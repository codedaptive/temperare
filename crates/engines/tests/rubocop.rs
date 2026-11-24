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
        let path = std::env::temp_dir().join(format!("engines-zit-{}-rubocop-{name}", std::process::id()));
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
fn rubocop_rule_reports_findings_clean_todo_and_absence() {
    let original = PathGuard(std::env::var_os("PATH"));
    std::env::set_var("PATH", std::env::join_paths(
        std::iter::once(Path::new("/opt/homebrew/bin").to_path_buf())
            .chain(std::env::split_paths(original.0.as_ref().unwrap()))
    ).unwrap());

    let run_case = |name| {
        let folder = TempFolder::new(name);
        git_support::git(&folder.0, &["init", "-q"], b"");
        let source = match name {
            "clean" => "# frozen_string_literal: true\n\ndef sample\n  nil\nend\n",
            _ => "# frozen_string_literal: true\n\ndef sample\n  first = 1\n  second = 2 # rubocop:disable Lint/UselessAssignment\n  nil\nend\n",
        };
        if name == "absent-no-ruby" {
            fs::write(folder.0.join("README.md"), "No Ruby files.\n").unwrap();
        } else {
            fs::write(folder.0.join("example.rb"), source).unwrap();
        }
        fs::write(folder.0.join("temperare.toml"), "version = 1\n[[rule]]\nid = \"unused-local\"\nengine = \"rubocop\"\ncheck = \"Lint/UselessAssignment\"\nfix = \"Remove the unused assignment.\"\n").unwrap();
        if name == "todo" {
            fs::write(folder.0.join(".rubocop.yml"), "inherit_from: .rubocop_todo.yml\n").unwrap();
            fs::write(folder.0.join(".rubocop_todo.yml"), "Lint/UselessAssignment:\n  Exclude:\n    - 'example.rb'\n").unwrap();
        }
        let file = RuleFile::read(&folder.0.join("temperare.toml")).unwrap();
        let _path_guard = if name == "absent" || name == "absent-no-ruby" {
            let guard = PathGuard(std::env::var_os("PATH"));
            let without_rubocop = std::env::join_paths(std::env::split_paths(guard.0.as_ref().unwrap()).filter(|dir| {
                !dir.join("rubocop").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
            })).unwrap();
            std::env::set_var("PATH", without_rubocop);
            Some(guard)
        } else { None };
        let runs = run_rules_versioned_at(&file, &folder.0, &[], None).runs;
        assert_eq!(runs.len(), 1, "{name}: {runs:?}");
        assert_eq!(runs[0].no_files, None, "{name}");
        match name {
            "absent" | "absent-no-ruby" => assert_eq!(runs[0].outcome, RuleOutcome::NotAvailable {
                rule: "unused-local".into(), engine: Engine::Rubocop, why: "rubocop not found on PATH".into(),
            }, "{name}"),
            "todo" => {
                let RuleOutcome::Failed { stderr, .. } = &runs[0].outcome else {
                    panic!("{name}: expected failure, got {:?}", runs[0].outcome);
                };
                assert!(stderr.contains(".rubocop_todo.yml"), "{name}: {stderr}");
            }
            _ => {
                let RuleOutcome::Findings(findings) = &runs[0].outcome else {
                    panic!("{name}: expected findings, got {:?}", runs[0].outcome);
                };
                let expected: &[(u32, u32, &str)] = if name == "clean" { &[] } else {
                    &[(4, 3, "Useless assignment to variable - `first`."),
                      (5, 3, "Useless assignment to variable - `second`.")]
                };
                assert_eq!(findings.len(), expected.len(), "{name}: {findings:?}");
                for (finding, (line, col, what)) in findings.iter().zip(expected) {
                    assert_eq!(finding.rule, "unused-local", "{name}");
                    assert_eq!(finding.path, RelPath::new("example.rb"), "{name}");
                    assert_eq!(finding.line(), Some(*line), "{name}");
                    assert_eq!(finding.col, Some(*col), "{name}");
                    assert_eq!(finding.what, *what, "{name}");
                    assert_eq!(finding.fix, "Remove the unused assignment.", "{name}");
                }
            }
        }
    };

    let mut failed = std::thread::scope(|scope| {
        let rows: Vec<_> = ["violation", "clean", "todo"].into_iter().map(|name| {
            let run_case = &run_case;
            (name, scope.spawn(move || std::panic::catch_unwind(|| run_case(name))))
        }).collect();
        rows.into_iter().filter_map(|(name, handle)| {
            if handle.join().unwrap().is_err() { Some(name) } else { None }
        }).collect::<Vec<_>>()
    });
    if std::panic::catch_unwind(|| run_case("absent")).is_err() { failed.push("absent"); }
    if std::panic::catch_unwind(|| run_case("absent-no-ruby")).is_err() { failed.push("absent-no-ruby"); }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
