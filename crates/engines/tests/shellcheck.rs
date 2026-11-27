#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use engines::{availability_in, entry_of, run_rules_versioned_at, Availability};
use rules::{Engine, RuleFile, RuleOutcome};

#[path = "support/git.rs"]
mod git_support;

struct TempFolder(PathBuf);

impl TempFolder {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("engines-zit-{}-shellcheck-entry", std::process::id()));
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

fn git(repo: &Path, args: &[&str]) {
    git_support::git(repo, args, b"");
}

#[test]
fn shellcheck_entry_reports_sc2086_finding_and_missing_program() {
    assert_eq!(entry_of(Engine::ShellCheck).name, "shellcheck");

    let repo = TempFolder::new();
    assert_eq!(availability_in(Engine::ShellCheck, &repo.0), Availability::Available);
    git(&repo.0, &["init", "-q"]);
    fs::write(repo.0.join("quote.sh"), "#!/bin/sh\nname=$1\necho $name\n").unwrap();
    fs::write(repo.0.join("tao.toml"), r#"version = 1
[[rule]]
id = "quote"
engine = "shellcheck"
check = "SC2086"
kind = "quality"
severity = "warning"
fix = "Quote the variable."
"#).unwrap();
    git(&repo.0, &["add", "quote.sh", "tao.toml"]);
    let file = RuleFile::read(&repo.0.join("tao.toml")).unwrap();
    let runs = run_rules_versioned_at(&file, &repo.0, &[], None).runs;
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].rule, "quote");
    assert_eq!(runs[0].engine, Engine::ShellCheck);
    assert!(matches!(&runs[0].outcome, RuleOutcome::Findings(_)), "expected Findings: {:?}", runs[0].outcome);
    let RuleOutcome::Findings(findings) = &runs[0].outcome else { unreachable!() };
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].rule, "quote");
    assert_eq!(findings[0].path, rules::RelPath::new("quote.sh"));
    assert_eq!(findings[0].line(), Some(3));
    assert_eq!(findings[0].what, "Double quote to prevent globbing and word splitting.");
    assert_eq!(findings[0].fix, "Quote the variable.");
    assert!(runs[0].raw_chars.is_some());

    // A clean script through this path is a separate case. A folder with no scripts is `NoFiles` with no check run;
    // src/shellcheck/integration/run_shellcheck_with_tests.rs (`repository_without_scripts_is_no_files_and_no_check_run`)
    // proves that flow.

    let path = PathGuard(std::env::var_os("PATH"));
    let without_shellcheck = std::env::join_paths(std::env::split_paths(path.0.as_ref().unwrap()).filter(|dir| {
        !dir.join("shellcheck").metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
    })).unwrap();
    std::env::set_var("PATH", without_shellcheck);
    let runs = run_rules_versioned_at(&file, &repo.0, &[], None).runs;
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0].rule, "quote");
    assert_eq!(runs[0].engine, Engine::ShellCheck);
    assert_eq!(runs[0].outcome, RuleOutcome::NotAvailable {
        rule: "quote".into(),
        engine: Engine::ShellCheck,
        why: "shellcheck not found on PATH".into(),
    });
    assert_eq!(runs[0].raw_chars, None);
    drop(path);
}
