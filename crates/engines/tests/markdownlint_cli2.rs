#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use engines::{run_rules_versioned_at, NotAvailableCause};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

#[path = "support/git.rs"]
mod git_support;

static LOCK: Mutex<()> = Mutex::new(());

struct TempFolder(PathBuf);

impl TempFolder {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-markdownlint-{name}", std::process::id()));
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
fn old_markdownlint_no_file_before_floor() {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let mut failed = Vec::new();
    for (name, file, old_floor) in [("no-file", "notes.txt", false), ("matching-file", "example.md", true)] {
        let result = std::panic::catch_unwind(|| {
            let folder = TempFolder::new(name);
            git_support::git(&folder.0, &["init", "-q"], b"");
            let bin = folder.0.join("bin");
            fs::create_dir(&bin).unwrap();
            for (program, script) in [
                ("markdownlint-cli2", "#!/bin/sh\necho 'markdownlint-cli2 v0.24.1 (markdownlint v0.42.1)'\nexit 2\n"),
                ("node", "#!/bin/sh\nexit 0\n"),
            ] {
                fs::write(bin.join(program), script).unwrap();
                fs::set_permissions(bin.join(program), fs::Permissions::from_mode(0o755)).unwrap();
            }
            let guard = PathGuard(std::env::var_os("PATH"));
            std::env::set_var("PATH", std::env::join_paths(std::iter::once(bin)
                .chain(std::env::split_paths(guard.0.as_ref().unwrap()))).unwrap());
            fs::write(folder.0.join(file), "example\n").unwrap();
            fs::write(folder.0.join("tao.toml"), "version = 1\n[[rule]]\nid = \"markdown-test\"\nengine = \"markdownlint\"\ncheck = \"MD013\"\npaths = [\"**\"]\nfix = \"Shorten the line.\"\n").unwrap();
            let rules = RuleFile::read(&folder.0.join("tao.toml")).unwrap();
            let checked = run_rules_versioned_at(&rules, &folder.0, &[], None);
            assert_eq!(checked.engines.len(), 1, "{name}: {:?}", checked.engines);
            assert_eq!(checked.engines[0].engine, Engine::Markdownlint, "{name}");
            assert_eq!(checked.engines[0].version.as_deref(), Some("0.24.1"), "{name}");
            assert_eq!(checked.runs.len(), 1, "{name}: {:?}", checked.runs);
            let run = &checked.runs[0];
            if old_floor {
                assert_eq!(run.no_files, None, "{name}: {run:?}");
                assert_eq!(run.not_available_cause, Some(NotAvailableCause::Other), "{name}: {run:?}");
                let RuleOutcome::NotAvailable { why, .. } = &run.outcome else { panic!("{name}: {run:?}") };
                assert_eq!(why, "markdownlint-cli2 0.24.1 is older than markdownlint-cli2 0.24.2, the release bundling markdownlint 0.42.2, which cctao's list of markdownlint codes was taken from", "{name}");
            } else {
                assert_eq!(run.outcome, RuleOutcome::Findings(vec![]), "{name}: {run:?}");
                assert_eq!(run.no_files.as_deref(), Some("no file its engine checks"), "{name}");
                assert_eq!(run.not_available_cause, None, "{name}");
            }
        });
        if result.is_err() { failed.push(name); } else { eprintln!("{name}: PASS"); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}

#[test]
fn markdownlint_rule_reports_findings_configs_and_availability() {
    let _lock = LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let original = PathGuard(std::env::var_os("PATH"));
    std::env::set_var("PATH", std::env::join_paths(
        std::iter::once(Path::new("/opt/homebrew/bin").to_path_buf())
            .chain(std::env::split_paths(original.0.as_ref().unwrap()))
    ).unwrap());
    let long = format!("{}words\n", "word ".repeat(19));
    let inline = format!("<!-- markdownlint-disable-next-line MD013 -->\n{long}");
    let fixable = format!("{}  \n", long.trim_end());
    let cases = [
        ("violation", "MD013"),
        ("package-json", "MD013"),
        ("alias", "line-length"),
        ("clean", "MD013"),
        ("other-rule", "MD013"),
        ("inline-disable", "MD013"),
        ("options", "MD013"),
        ("options-below", "MD013"),
        ("config-off-json", "MD013"),
        ("config-off-yaml-alias", "MD013"),
        ("config-default-off", "MD013"),
        ("config-on-options", "MD013"),
        ("config-jsonc", "MD013"),
        ("unknown", "MD999"),
        ("malformed", "MD 013"),
        ("no-markdown", "MD013"),
        ("no-markdown-absent", "MD013"),
        ("absent", "MD013"),
    ];
    let mut failed = Vec::new();
    for (name, check) in cases {
        let result = std::panic::catch_unwind(|| {
            let folder = TempFolder::new(name);
            git_support::git(&folder.0, &["init", "-q"], b"");
            let path = if name == "options-below" { "sub/example.md" }
                else if name.starts_with("no-markdown") { "notes.txt" }
                else { "example.md" };
            let source = match name {
                "clean" => "A clean line.\n",
                "other-rule" => "Trailing spaces.  \n",
                "inline-disable" => &inline,
                "options" | "options-below" => &fixable,
                "no-markdown" | "no-markdown-absent" => "No Markdown here.\n",
                _ => &long,
            };
            let source_path = folder.0.join(path);
            fs::create_dir_all(source_path.parent().unwrap()).unwrap();
            fs::write(&source_path, source).unwrap();
            let config = match name {
                "package-json" => Some(("package.json", r#"{"markdownlint-cli2":{"config":{"MD013":false}},"markdownlint":{"MD013":false}}"#)),
                "options" => Some((".markdownlint-cli2.jsonc", "{ \"fix\": true }")),
                "options-below" => Some(("sub/.markdownlint-cli2.jsonc", "{ \"fix\": true }")),
                "config-off-json" => Some((".markdownlint.json", "{\"MD013\": false}")),
                "config-off-yaml-alias" => Some((".markdownlint.yaml", "line-length: false\n")),
                "config-default-off" => Some((".markdownlint.json", "{\"default\": false}")),
                "config-on-options" => Some((".markdownlint.json", "{\"MD013\": {\"line_length\": 200}}")),
                "config-jsonc" => Some((".markdownlint.jsonc", "{}")),
                _ => None,
            };
            if let Some((config_path, contents)) = config {
                fs::write(folder.0.join(config_path), contents).unwrap();
            }
            fs::write(folder.0.join("tao.toml"), format!(
                "version = 1\n[[rule]]\nid = \"markdown-test\"\nengine = \"markdownlint\"\ncheck = \"{check}\"\nfix = \"Shorten the line.\"\n"
            )).unwrap();
            let file = RuleFile::read(&folder.0.join("tao.toml")).unwrap();
            let _path_guard = if name == "absent" || name == "no-markdown-absent" {
                let guard = PathGuard(std::env::var_os("PATH"));
                let without_engine = std::env::join_paths(std::env::split_paths(guard.0.as_ref().unwrap()).filter(|dir| {
                    !dir.join("markdownlint-cli2").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
                })).unwrap();
                std::env::set_var("PATH", without_engine);
                Some(guard)
            } else { None };
            let runs = run_rules_versioned_at(&file, &folder.0, &[], None).runs;
            assert_eq!(runs.len(), 1, "{name}: {runs:?}");
            let run = &runs[0];
            if name == "options" || name == "options-below" {
                assert_eq!(fs::read(&source_path).unwrap(), source.as_bytes(), "{name}: Markdown was modified");
            }
            match name {
                "options" | "options-below" | "config-jsonc" | "malformed" => {
                    let RuleOutcome::Failed { stderr, .. } = &run.outcome else {
                        panic!("{name}: expected Failed, got {:?}", run.outcome);
                    };
                    let token = match name {
                        "malformed" => "MD 013",
                        "config-jsonc" => ".markdownlint.jsonc",
                        _ => ".markdownlint-cli2.jsonc",
                    };
                    assert!(stderr.contains(token), "{name}: {stderr}");
                }
                "config-off-json" | "config-off-yaml-alias" | "config-default-off" | "unknown" | "absent" => {
                    let RuleOutcome::NotAvailable { rule, engine, why } = &run.outcome else {
                        panic!("{name}: expected NotAvailable, got {:?}", run.outcome);
                    };
                    assert_eq!(rule, "markdown-test", "{name}");
                    assert_eq!(*engine, Engine::Markdownlint, "{name}");
                    let token = match name {
                        "config-off-json" | "config-default-off" => ".markdownlint.json",
                        "config-off-yaml-alias" => ".markdownlint.yaml",
                        "unknown" => "MD999",
                        _ => "PATH",
                    };
                    assert!(why.contains(token), "{name}: {why}");
                    let setting = match name {
                        "config-off-json" => Some("MD013"),
                        "config-off-yaml-alias" => Some("line-length"),
                        "config-default-off" => Some("default"),
                        "absent" => Some("not found"),
                        _ => None,
                    };
                    if let Some(setting) = setting { assert!(why.contains(setting), "{name}: {why}"); }
                }
                _ => {
                    let RuleOutcome::Findings(findings) = &run.outcome else {
                        panic!("{name}: expected findings, got {:?}", run.outcome);
                    };
                    if name.starts_with("no-markdown") {
                        assert_eq!(run.no_files.as_deref(), Some("no file its engine checks"), "{name}");
                    } else {
                        assert_eq!(run.no_files, None, "{name}");
                    }
                    let count = usize::from(matches!(name, "violation" | "package-json" | "alias" | "inline-disable"));
                    assert_eq!(findings.len(), count, "{name}: {findings:?}");
                    if count == 1 {
                        let finding = &findings[0];
                        assert_eq!(finding.rule, "markdown-test", "{name}");
                        assert_eq!(finding.path, RelPath::new(path), "{name}");
                        assert_eq!(finding.line(), Some(if name == "inline-disable" { 2 } else { 1 }), "{name}");
                        assert!(finding.what.contains("MD013/line-length"), "{name}: {}", finding.what);
                        assert!(finding.what.contains("Line length"), "{name}: {}", finding.what);
                        assert_eq!(finding.fix, "Shorten the line.", "{name}");
                    }
                }
            }
        });
        if result.is_err() { failed.push(name); } else { eprintln!("{name}: PASS"); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
