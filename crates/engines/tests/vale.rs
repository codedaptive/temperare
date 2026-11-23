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
        let path = std::env::temp_dir().join(format!("engines-zit-{}-vale-{name}", std::process::id()));
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
fn vale_rule_reports_findings_config_and_availability() {
    let original = PathGuard(std::env::var_os("PATH"));
    std::env::set_var("PATH", std::env::join_paths(
        std::iter::once(Path::new("/opt/homebrew/bin").to_path_buf())
            .chain(std::env::split_paths(original.0.as_ref().unwrap()))
    ).unwrap());
    let cases = [
        ("violation", "Vale.Spelling"),
        ("rule-config", "Vale.Spelling"),
        ("clean", "Vale.Spelling"),
        ("absent", "Vale.Spelling"),
        ("no-config", "Vale.Spelling"),
        ("unknown-check", "Vale.Speling"),
        ("style-not-named", "Other.Rule"),
        ("block-ignores-off", "Vale.Spelling"),
        ("vocab-on", "Vale.Spelling"),
        ("no-prose-file", "Vale.Spelling"),
        ("no-prose-file-absent", "Vale.Spelling"),
        ("style-missing-on-disk", "Gone.Rule"),
        ("gitignored", "Vale.Spelling"),
    ];
    let mut failed = Vec::new();
    for (name, check) in cases {
        let result = std::panic::catch_unwind(|| {
            let folder = TempFolder::new(name);
            git_support::git(&folder.0, &["init", "-q"], b"");
            fs::create_dir(folder.0.join("styles")).unwrap();
            let no_prose = name == "no-prose-file" || name == "no-prose-file-absent";
            if no_prose {
                fs::write(folder.0.join("data.json"), "{}\n").unwrap();
            } else {
                let prose = if name == "clean" { "A sentence here.\n" } else { "A sentense here.\n" };
                fs::write(folder.0.join("doc.md"), prose).unwrap();
            }
            if name == "gitignored" {
                fs::write(folder.0.join(".gitignore"), "ignored.md\n").unwrap();
                fs::write(folder.0.join("ignored.md"), "A sentense here.\n").unwrap();
            }
            if name == "vocab-on" {
                let vocab = folder.0.join("styles/config/vocabularies/Proj");
                fs::create_dir_all(&vocab).unwrap();
                fs::write(vocab.join("accept.txt"), "sentense\n").unwrap();
            }
            if name != "no-config" {
                let vocab = if name == "vocab-on" { "Vocab = Proj\n" } else { "" };
                let styles = if name == "style-missing-on-disk" { "Vale, Gone" } else { "Vale" };
                let ignores = if name == "block-ignores-off" { "BlockIgnores = (?s)A sentense.*\n" } else { "" };
                let config_name = if name == "rule-config" { "docs.ini" } else { ".vale.ini" };
                fs::write(folder.0.join(config_name), format!(
                    "StylesPath = styles\n{vocab}[*.md]\nBasedOnStyles = {styles}\n{ignores}"
                )).unwrap();
            }
            fs::write(folder.0.join("temperare.toml"), format!(
                "version = 1\n[[rule]]\nid = \"prose-spelling\"\nengine = \"vale\"\ncheck = \"{check}\"\npaths = [\"**\"]\nfix = \"Correct the prose.\"\n{}",
                if name == "rule-config" { "config = \"docs.ini\"\n" } else { "" }
            )).unwrap();
            let file = RuleFile::read(&folder.0.join("temperare.toml")).unwrap();
            let _path_guard = if name == "absent" || name == "no-prose-file-absent" {
                let guard = PathGuard(std::env::var_os("PATH"));
                let without_vale = std::env::join_paths(std::env::split_paths(guard.0.as_ref().unwrap()).filter(|dir| {
                    !dir.join("vale").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
                })).unwrap();
                std::env::set_var("PATH", without_vale);
                Some(guard)
            } else { None };
            let runs = run_rules_versioned_at(&file, &folder.0, &[], None).runs;
            assert_eq!(runs.len(), 1, "{name}: {runs:?}");
            let run = &runs[0];
            if no_prose {
                assert_eq!(run.no_files.as_deref(), Some("no file its engine checks"), "{name}: {run:?}");
                assert_eq!(run.outcome, RuleOutcome::Findings(vec![]), "{name}");
                return;
            }
            assert_eq!(run.no_files, None, "{name}: {run:?}");
            match name {
                "absent" | "no-config" | "unknown-check" | "style-not-named" => {
                    let RuleOutcome::NotAvailable { rule, engine, why } = &run.outcome else {
                        panic!("{name}: expected NotAvailable, got {:?}", run.outcome);
                    };
                    assert_eq!(rule, "prose-spelling", "{name}");
                    assert_eq!(*engine, Engine::Vale, "{name}");
                    if name == "no-config" { assert!(why.contains(".vale.ini"), "{name}: {why}"); }
                    if name == "absent" { assert!(why.contains("not found on PATH"), "{name}: {why}"); }
                    if name == "style-not-named" { assert!(why.contains("Other"), "{name}: {why}"); }
                    if name == "unknown-check" {
                        assert_eq!(run.not_available_cause, Some(NotAvailableCause::UnknownCheck), "{name}: {run:?}");
                    }
                }
                "style-missing-on-disk" => {
                    let RuleOutcome::Failed { stderr, .. } = &run.outcome else {
                        panic!("{name}: expected Failed, got {:?}", run.outcome);
                    };
                    assert!(stderr.contains("Gone"), "{name}: {stderr}");
                }
                _ => {
                    let RuleOutcome::Findings(findings) = &run.outcome else {
                        panic!("{name}: expected findings, got {:?}", run.outcome);
                    };
                    let count = usize::from(name != "clean" && name != "vocab-on");
                    assert_eq!(findings.len(), count, "{name}: {findings:?}");
                    if count == 1 {
                        let finding = &findings[0];
                        assert_eq!(finding.rule, "prose-spelling", "{name}");
                        assert_eq!(finding.path, RelPath::new("doc.md"), "{name}");
                        assert_eq!(finding.line(), Some(1), "{name}");
                        assert_eq!(finding.col, Some(3), "{name}");
                        assert!(finding.what.contains("sentense"), "{name}: {}", finding.what);
                        assert_eq!(finding.fix, "Correct the prose.", "{name}");
                    }
                }
            }
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
