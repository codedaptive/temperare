#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use engines::run_rules_versioned_at;
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

#[path = "support/git.rs"]
mod git_support;

struct TempFolder(PathBuf);

impl TempFolder {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-phpstan-{name}", std::process::id()));
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
fn phpstan_rule_reports_levels_suppressions_and_availability() {
    let original = PathGuard(std::env::var_os("PATH"));
    std::env::set_var("PATH", std::env::join_paths(
        std::iter::once(Path::new("/opt/homebrew/bin").to_path_buf())
            .chain(std::env::split_paths(original.0.as_ref().unwrap()))
    ).unwrap());
    let violation = "<?php\nfunction takeInt(int $value): void {}\ntakeInt('wrong');\n";
    let cases = [
        ("violation", "5"),
        ("inline-ignore", "5"),
        ("max", "max"),
        ("clean", "5"),
        ("ignored", "5"),
        ("excluded", "5"),
        ("unknown-level", "11"),
        ("malformed", "five"),
        ("no-php", "5"),
        ("no-php-absent", "5"),
        ("no-config", "5"),
        ("absent", "5"),
    ];
    let mut failed = Vec::new();
    for (name, check) in cases {
        let result = std::panic::catch_unwind(|| {
            let folder = TempFolder::new(name);
            git_support::git(&folder.0, &["init", "-q"], b"");
            let no_php = name == "no-php" || name == "no-php-absent";
            if no_php {
                fs::write(folder.0.join("notes.txt"), "no PHP here\n").unwrap();
            } else {
                fs::create_dir(folder.0.join("src")).unwrap();
                if name == "max" {
                    fs::write(folder.0.join("src/max.php"), "<?php\nfunction printValue($value): void {\n    echo $value;\n}\n").unwrap();
                } else {
                    let source = if name == "clean" { "<?php\nfunction takeInt(int $value): void {}\ntakeInt(1);\n" } else if name == "inline-ignore" { "<?php\nfunction takeInt(int $value): void {}\ntakeInt('wrong'); // @phpstan-ignore argument.type\n" } else { violation };
                    fs::write(folder.0.join("src/example.php"), source).unwrap();
                }
            }
            if name == "violation" || name == "inline-ignore" {
                fs::write(folder.0.join(".gitignore"), "src/ignored.php\n").unwrap();
                fs::write(folder.0.join("src/ignored.php"), violation).unwrap();
                fs::write(folder.0.join("outside.php"), violation).unwrap();
            }
            if name == "excluded" {
                fs::write(folder.0.join("src/excluded.php"), "<?php\nfunction takeOtherInt(int $value): void {}\ntakeOtherInt('wrong');\n").unwrap();
            }
            if name == "ignored" {
                git_support::git(&folder.0, &["add", "src/example.php"], b"");
                fs::write(folder.0.join("phpstan-baseline.neon"),
                    "parameters:\n    ignoreErrors:\n        -\n            identifier: argument.type\n            path: src/example.php\n").unwrap();
            }
            if name != "no-config" {
                let suppressions = if name == "ignored" {
                    "includes:\n    - phpstan-baseline.neon\nparameters:\n    level: 0\n    paths:\n        - src\n    ignoreErrors:\n        -\n            identifier: argument.type\n"
                } else if name == "excluded" {
                    "parameters:\n    level: 0\n    paths:\n        - src\n    excludePaths:\n        - src/excluded.php\n"
                } else {
                    "parameters:\n    level: 0\n    paths:\n        - src\n"
                };
                fs::write(folder.0.join("phpstan.neon"), suppressions).unwrap();
            }
            fs::write(folder.0.join("temperare.toml"), format!(
                "version = 1\n[[rule]]\nid = \"argument-type\"\nengine = \"phpstan\"\ncheck = \"{check}\"\n{}fix = \"Pass an integer.\"\n",
                if no_php { "" } else { "paths = [\"src/**\"]\n" }
            )).unwrap();
            let file = RuleFile::read(&folder.0.join("temperare.toml")).unwrap();
            let _path_guard = if name == "absent" || name == "no-php-absent" {
                let guard = PathGuard(std::env::var_os("PATH"));
                let without_phpstan = std::env::join_paths(std::env::split_paths(guard.0.as_ref().unwrap()).filter(|dir| {
                    !dir.join("phpstan").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
                })).unwrap();
                std::env::set_var("PATH", without_phpstan);
                Some(guard)
            } else { None };
            let runs = run_rules_versioned_at(&file, &folder.0, &[], None).runs;
            assert_eq!(runs.len(), 1, "{name}: {runs:?}");
            let run = &runs[0];
            if no_php {
                assert_eq!(run.no_files.as_deref(), Some("no file its engine checks"), "{name}: {run:?}");
                assert_eq!(run.outcome, RuleOutcome::Findings(vec![]), "{name}");
                return;
            }
            match name {
                "unknown-level" | "no-config" | "absent" => {
                    let RuleOutcome::NotAvailable { rule, engine, why } = &run.outcome else {
                        panic!("{name}: expected NotAvailable, got {:?}", run.outcome);
                    };
                    assert_eq!(rule, "argument-type", "{name}");
                    assert_eq!(*engine, Engine::Phpstan, "{name}");
                    match name {
                        "unknown-level" => assert!(why.starts_with("phpstan 2.3.7 does not know the check `11`"), "{name}: {why}"),
                        "no-config" => assert!(why.contains("phpstan.neon"), "{name}: {why}"),
                        _ => assert_eq!(why, "phpstan not found on PATH", "{name}"),
                    }
                }
                "malformed" => {
                    let RuleOutcome::Failed { stderr, .. } = &run.outcome else {
                        panic!("{name}: expected Failed, got {:?}", run.outcome);
                    };
                    assert!(stderr.contains("is not a PHPStan level"), "{name}: {stderr}");
                }
                _ => {
                    let RuleOutcome::Findings(findings) = &run.outcome else {
                        panic!("{name}: expected findings, got {:?}", run.outcome);
                    };
                    let count = if name == "excluded" || name == "max" { 2 } else { usize::from(name != "clean" && name != "inline-ignore") };
                    assert_eq!(findings.len(), count, "{name}: {findings:?}");
                    if name == "inline-ignore" { assert_eq!(run.no_files, None, "{name}"); }
                    if name == "max" {
                        let finding = findings.iter().find(|f| f.what == "Parameter #1 (mixed) of echo cannot be converted to string.")
                            .unwrap_or_else(|| panic!("{name}: missing max-level finding: {findings:?}"));
                        assert_eq!(finding.path, RelPath::new("src/max.php"), "{name}");
                        assert_eq!(finding.line(), Some(3), "{name}");
                    }
                    if name == "excluded" {
                        let finding = findings.iter().find(|f| f.what == "Parameter #1 $value of function takeOtherInt expects int, string given.")
                            .unwrap_or_else(|| panic!("{name}: missing excluded finding: {findings:?}"));
                        assert_eq!(finding.path, RelPath::new("src/excluded.php"), "{name}");
                        assert_eq!(finding.line(), Some(3), "{name}");
                    }
                    if name != "clean" && name != "max" && name != "inline-ignore" {
                        let finding = findings.iter().find(|f| f.what == "Parameter #1 $value of function takeInt expects int, string given.")
                            .unwrap_or_else(|| panic!("{name}: missing argument.type finding: {findings:?}"));
                        assert_eq!(finding.rule, "argument-type", "{name}");
                        assert_eq!(finding.path, RelPath::new("src/example.php"), "{name}");
                        assert_eq!(finding.line(), Some(3), "{name}");
                        assert_eq!(finding.col, None, "{name}");
                        assert_eq!(finding.fix, "Pass an integer.", "{name}");
                    }
                }
            }
            if name == "violation" {
                let mut entries: Vec<_> = fs::read_dir(&folder.0).unwrap().map(|e| e.unwrap().file_name()).collect();
                entries.sort();
                assert_eq!(entries, [".git", ".gitignore", "outside.php", "phpstan.neon", "src", "temperare.toml"]);
                let mut src_entries: Vec<_> = fs::read_dir(folder.0.join("src")).unwrap().map(|e| e.unwrap().file_name()).collect();
                src_entries.sort();
                assert_eq!(src_entries, ["example.php", "ignored.php"]);
            }
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
