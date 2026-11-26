#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use engines::{run_rules_versioned_at, NotAvailableCause};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

#[path = "support/git.rs"]
mod git_support;

struct TempFolder(PathBuf);

impl TempFolder {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-yamllint-{name}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) { fs::remove_dir_all(&self.0).unwrap(); }
}

struct PathGuard(Option<OsString>);

impl Drop for PathGuard {
    fn drop(&mut self) {
        match &self.0 {
            Some(path) => std::env::set_var("PATH", path),
            None => std::env::remove_var("PATH"),
        }
    }
}

#[test]
fn yamllint_rule_reports_findings_failures_and_availability() {
    let original = PathGuard(std::env::var_os("PATH"));
    std::env::set_var("PATH", std::env::join_paths(
        std::iter::once(Path::new("/opt/homebrew/bin").to_path_buf())
            .chain(std::env::split_paths(original.0.as_ref().unwrap()))
    ).unwrap());

    let cases = [
        ("violation", "truthy"),
        ("clean", "truthy"),
        ("disabled-in-config", "truthy"),
        ("rule-ignore", "truthy"),
        ("top-level-ignore", "truthy"),
        ("inherited-ignore", "truthy"),
        ("ignore-from-file", "truthy"),
        ("check-ignore-from-file", "truthy"),
        ("unknown-check", "no-such-rule"),
        ("malformed-check", "Truthy!"),
        ("syntax-error", "truthy"),
        ("no-yaml", "truthy"),
        ("no-yaml-absent", "truthy"),
        ("absent", "truthy"),
        ("missing-config", "truthy"),
    ];
    let mut failed = Vec::new();
    for (name, check) in cases {
        let result = std::panic::catch_unwind(|| {
            let folder = TempFolder::new(name);
            git_support::git(&folder.0, &["init", "-q"], b"");
            let no_yaml = name == "no-yaml" || name == "no-yaml-absent";
            match name {
                "no-yaml" | "no-yaml-absent" => fs::write(folder.0.join("notes.txt"), "notes\n").unwrap(),
                "syntax-error" => fs::write(folder.0.join("bad.yaml"), "a: [1, 2\n").unwrap(),
                "clean" => fs::write(folder.0.join("a.yaml"), "---\nkey: true\n").unwrap(),
                _ => fs::write(folder.0.join("a.yaml"), "key: yes\n").unwrap(),
            }
            let config = match name {
                "violation" => Some("extends: default\n"),
                "disabled-in-config" => Some("extends: default\nrules:\n  truthy: disable\n"),
                "rule-ignore" => Some("extends: default\nrules:\n  truthy:\n    ignore: a.yaml\n"),
                "top-level-ignore" => Some("extends: default\nignore: |\n  a.yaml\n"),
                "inherited-ignore" => Some("extends: base.yaml\n"),
                "ignore-from-file" => Some("extends: default\nignore-from-file: ign.txt\n"),
                "check-ignore-from-file" => Some("extends: default\nrules:\n  truthy:\n    ignore-from-file: ign.txt\n"),
                _ => None,
            };
            if let Some(config) = config { fs::write(folder.0.join(".yamllint"), config).unwrap(); }
            if name == "inherited-ignore" { fs::write(folder.0.join("base.yaml"), "extends: default\nignore: |\n  a.yaml\n").unwrap(); }
            if name == "ignore-from-file" || name == "check-ignore-from-file" { fs::write(folder.0.join("ign.txt"), "a.yaml\n").unwrap(); }
            fs::write(folder.0.join("temperare.toml"), format!(
                "version = 1\n[[rule]]\nid = \"yaml-check\"\nengine = \"yamllint\"\ncheck = \"{check}\"\n{}fix = \"Fix YAML.\"\n",
                if name == "missing-config" { "config = \"missing.yaml\"\n" } else { "" }
            )).unwrap();
            let file = RuleFile::read(&folder.0.join("temperare.toml")).unwrap();
            let _path_guard = if name == "absent" || name == "no-yaml-absent" {
                let guard = PathGuard(std::env::var_os("PATH"));
                let without_yamllint = std::env::join_paths(std::env::split_paths(guard.0.as_ref().unwrap()).filter(|dir| {
                    !dir.join("yamllint").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
                })).unwrap();
                std::env::set_var("PATH", without_yamllint);
                Some(guard)
            } else { None };
            let runs = run_rules_versioned_at(&file, &folder.0, &[], None).runs;
            assert_eq!(runs.len(), 1, "{name}: {runs:?}");
            let run = &runs[0];
            if no_yaml {
                assert_eq!(run.no_files.as_deref(), Some("no file its engine checks"), "{name}: {run:?}");
                assert_eq!(run.outcome, RuleOutcome::Findings(vec![]), "{name}");
                return;
            }
            match name {
                "unknown-check" | "absent" => {
                    let RuleOutcome::NotAvailable { rule, engine, why } = &run.outcome else {
                        panic!("{name}: expected NotAvailable, got {:?}", run.outcome);
                    };
                    assert_eq!(rule, "yaml-check", "{name}");
                    assert_eq!(*engine, Engine::Yamllint, "{name}");
                    let (cause, token) = if name == "unknown-check" {
                        (NotAvailableCause::UnknownCheck, "no-such-rule")
                    } else {
                        (NotAvailableCause::EngineNotOnPath, "yamllint")
                    };
                    assert_eq!(run.not_available_cause, Some(cause), "{name}: {run:?}");
                    assert!(why.contains(token), "{name}: {why}");
                }
                "malformed-check" | "syntax-error" | "missing-config" => {
                    let RuleOutcome::Failed { stderr, .. } = &run.outcome else {
                        panic!("{name}: expected Failed, got {:?}", run.outcome);
                    };
                    let token = match name {
                        "malformed-check" => Some("is not a yamllint rule id"),
                        "syntax-error" => Some("bad.yaml"),
                        "missing-config" => Some("missing.yaml"),
                        _ => None,
                    };
                    if let Some(token) = token { assert!(stderr.contains(token), "{name}: {stderr}"); }
                }
                _ => {
                    let RuleOutcome::Findings(findings) = &run.outcome else {
                        panic!("{name}: expected Findings, got {:?}", run.outcome);
                    };
                    assert_eq!(findings.len(), usize::from(name != "clean"), "{name}: {findings:?}");
                    if let Some(finding) = findings.first() {
                        assert_eq!(finding.rule, "yaml-check", "{name}");
                        assert_eq!(finding.fix, "Fix YAML.", "{name}");
                        assert_eq!(finding.path, RelPath::new("a.yaml"), "{name}");
                        assert_eq!(finding.line(), Some(1), "{name}");
                        assert_eq!(finding.col, Some(6), "{name}");
                        assert!(finding.what.starts_with("truthy value should be one of"), "{name}: {}", finding.what);
                        assert!(!finding.what.ends_with(" (truthy)"), "{name}: {}", finding.what);
                    }
                }
            }
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
