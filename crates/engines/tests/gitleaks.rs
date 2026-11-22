#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use engines::run_rules_versioned_at;
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

struct TempFolder(PathBuf);

impl TempFolder {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-gitleaks", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(path).unwrap())
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
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
fn gitleaks_rule_reports_secret_clean_unknown_and_absence() {
    let folder = TempFolder::new();
    let original_path = PathGuard(std::env::var_os("PATH"));
    std::env::set_var("PATH", std::env::join_paths(
        std::iter::once(Path::new("/opt/homebrew/bin").to_path_buf())
            .chain(std::env::split_paths(original_path.0.as_deref().unwrap_or_default()))
    ).unwrap());
    let key = format!("AKIA{}", "Q7Z2M4X6W3B6N5KD");
    let secret_source = format!("aws_key = \"{key}\"\n");
    let cases = [
        ("secret", secret_source.as_str(), "aws-access-token", false),
        ("clean", "VALUE = 1\n", "aws-access-token", false),
        ("unknown rule", secret_source.as_str(), "no-such-rule", false),
        ("gitleaks absent", secret_source.as_str(), "aws-access-token", true),
    ];
    let mut failed = Vec::new();
    for (name, source, check, absent) in cases {
        let row = std::panic::catch_unwind(|| {
            fs::write(folder.0.join("config.py"), source).unwrap();
            fs::write(folder.0.join("temperare.toml"), format!(
                "version = 1\nowner = \"security-team\"\n[[rule]]\nid = \"aws-secret\"\nengine = \"gitleaks\"\nkind = \"security\"\ncheck = \"{check}\"\nfix = \"Remove the exposed key.\"\n"
            )).unwrap();
            let file = RuleFile::read(&folder.0.join("temperare.toml")).unwrap();
            let _path_guard = if absent {
                let guard = PathGuard(std::env::var_os("PATH"));
                let without_gitleaks = std::env::join_paths(
                    std::env::split_paths(guard.0.as_ref().unwrap()).filter(|dir| {
                        !dir.join("gitleaks").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
                    })
                ).unwrap();
                std::env::set_var("PATH", without_gitleaks);
                Some(guard)
            } else {
                None
            };
            let result = run_rules_versioned_at(&file, &folder.0, &[], None);
            let debug = format!("{result:?}");
            assert!(!debug.contains(&key), "{name}: check result exposed the key");
            assert_eq!(result.runs.len(), 1, "{name}: expected one run");
            let outcome = &result.runs[0].outcome;
            match name {
                "secret" | "clean" => {
                    let RuleOutcome::Findings(findings) = outcome else {
                        panic!("{name}: expected Findings, got {}", format!("{outcome:?}").replace(&key, "[redacted]"));
                    };
                    let count = if name == "secret" { 1 } else { 0 };
                    assert_eq!(findings.len(), count, "{name}: finding count");
                    if name == "secret" {
                        let finding = &findings[0];
                        assert!(!format!("{finding:?}").contains(&key), "{name}: finding exposed the key");
                        assert!(finding.rule == "aws-secret", "{name}: rule id");
                        assert!(finding.path == RelPath::new("config.py"), "{name}: path");
                        assert_eq!(finding.line(), Some(1), "{name}: line");
                        assert_eq!(finding.col, Some(12), "{name}: column");
                        assert!(finding.fix == "Remove the exposed key.", "{name}: fix");
                    }
                }
                "unknown rule" | "gitleaks absent" => {
                    let RuleOutcome::NotAvailable { rule, engine, why } = outcome else {
                        panic!("{name}: expected NotAvailable, got {}", format!("{outcome:?}").replace(&key, "[redacted]"));
                    };
                    assert!(rule == "aws-secret", "{name}: rule id");
                    assert_eq!(*engine, Engine::Gitleaks, "{name}: engine");
                    if absent {
                        assert!(why == "gitleaks not found on PATH", "{name}: {}", why.replace(&key, "[redacted]"));
                    } else {
                        assert!(why.contains("Requested rule no-such-rule not found in rules"), "{name}: {}", why.replace(&key, "[redacted]"));
                    }
                }
                _ => unreachable!(),
            }
        });
        if row.is_err() {
            failed.push(name);
        }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
