//! ZUT for run_rules.rs: the listing, the problems and the discovery errors, as text and as JSON, with git, the disk
//! and engine availability as closures and rule files parsed from literal TOML.

use super::{run_rules, RulesWorld};
use crate::RulesArgs;
use engines::Availability;
use procio::GitError;
use rules::{Engine, Problem, RuleFile};
use serde_json::json;
use std::path::{Path, PathBuf};

const GOOD: &str = r#"version = 1

[[rule]]
id = "lint-all"
engine = "clippy"
check = "clippy::all"
fix = "Do what clippy says."

[[exception]]
rule = "lint-all"
path = "src/legacy.rs"
reason = "Legacy."
approved = "bob 2026-11-17"
"#;

fn good(path: &Path) -> Result<RuleFile, Vec<Problem>> {
    assert_eq!(path, Path::new("/repo/temperare.toml"));
    Ok(RuleFile::parse(GOOD, Path::new(".")).unwrap())
}

fn broken(_: &Path) -> Result<RuleFile, Vec<Problem>> {
    Err(vec![
        Problem { line: 1, col: 11, what: "version must be 1".to_string(), fix: "Write version = 1.".to_string() },
        Problem { line: 3, col: 1, what: "unknown key".to_string(), fix: "Remove the key.".to_string() },
    ])
}

fn repo_top(cwd: &Path) -> Result<PathBuf, GitError> {
    assert_eq!(cwd, Path::new("/repo/src"));
    Ok(PathBuf::from("/repo"))
}

fn not_a_repository(cwd: &Path) -> Result<PathBuf, GitError> {
    Err(GitError::NotARepository { repo: cwd.to_path_buf() })
}

fn there(_: &Path) -> bool {
    true
}

fn all_available(_: Engine) -> Availability {
    Availability::Available
}

/// Runs `tm rules` in /repo/src with the given seams, every engine available; (exit, out, err).
fn rules_in_repo(
    args: RulesArgs,
    toplevel: &dyn Fn(&Path) -> Result<PathBuf, GitError>,
    exists: &dyn Fn(&Path) -> bool,
    read: &dyn Fn(&Path) -> Result<RuleFile, Vec<Problem>>,
) -> (u8, String, String) {
    rules_with(args, toplevel, exists, read, &all_available)
}

/// Runs `tm rules` in /repo/src with the given seams and engine availability; (exit, out, err).
fn rules_with(
    args: RulesArgs,
    toplevel: &dyn Fn(&Path) -> Result<PathBuf, GitError>,
    exists: &dyn Fn(&Path) -> bool,
    read: &dyn Fn(&Path) -> Result<RuleFile, Vec<Problem>>,
    availability: &dyn Fn(Engine) -> Availability,
) -> (u8, String, String) {
    let world = RulesWorld { toplevel, exists, read, availability };
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = run_rules(&args, Path::new("/repo/src"), &world, &mut out, &mut err);
    (code, String::from_utf8(out).unwrap(), String::from_utf8(err).unwrap())
}

fn json_args() -> RulesArgs {
    RulesArgs { rules: None, json: true }
}

#[test]
fn text_lists_the_rules_then_nothing_to_do() {
    let (code, out, err) = rules_in_repo(RulesArgs::default(), &repo_top, &there, &good);
    assert_eq!(code, 0);
    assert_eq!(
        out,
        "rule file: /repo/temperare.toml\n\
         rules (1):\n  \
         lint-all  clippy  quality  error  paths: all files  available\n\
         exceptions in force (1):\n  \
         lint-all  src/legacy.rs  approved: bob 2026-11-17  reason: Legacy.\n\
         next: nothing to do\n"
    );
    assert_eq!(err, "");
}

#[test]
fn json_lists_the_rules_with_exit_0_as_one_object() {
    let (code, out, err) = rules_in_repo(json_args(), &repo_top, &there, &good);
    assert_eq!(code, 0);
    assert!(out.ends_with("}\n"), "{out}");
    let value: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(
        value,
        json!({
            "rule_file": "/repo/temperare.toml",
            "rules": [{
                "id": "lint-all", "engine": "clippy", "kind": "quality", "severity": "error",
                "check": "clippy::all", "paths": [], "exclude": [], "kits": [], "outside_kits": [],
                "available": true, "why": null,
            }],
            "exceptions": [{
                "rule": "lint-all", "path": "src/legacy.rs", "reason": "Legacy.",
                "approved": "bob 2026-11-17",
            }],
            "exceptions_in_force": 1,
            "exit": 0,
            "next": "nothing to do",
        })
    );
    assert_eq!(err, "");
}

#[test]
fn text_problems_one_line_each_then_the_fix_with_exit_2() {
    let (code, out, err) = rules_in_repo(RulesArgs::default(), &repo_top, &there, &broken);
    assert_eq!(code, 2);
    assert_eq!(
        out,
        "/repo/temperare.toml:1:11  version must be 1  Write version = 1.\n\
         /repo/temperare.toml:3:1  unknown key  Remove the key.\n\
         next: fix each problem listed for /repo/temperare.toml\n"
    );
    assert_eq!(err, "");
}

#[test]
fn json_problems_with_exit_2_and_the_fix() {
    let (code, out, _) = rules_in_repo(json_args(), &repo_top, &there, &broken);
    assert_eq!(code, 2);
    let value: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(
        value,
        json!({
            "rule_file": "/repo/temperare.toml",
            "problems": [
                {"line": 1, "col": 11, "what": "version must be 1", "fix": "Write version = 1."},
                {"line": 3, "col": 1, "what": "unknown key", "fix": "Remove the key."},
            ],
            "exit": 2,
            "next": "fix each problem listed for /repo/temperare.toml",
        })
    );
}

#[test]
fn outside_a_repository_is_an_error_on_stderr_and_nothing_on_stdout() {
    let (code, out, err) = rules_in_repo(RulesArgs::default(), &not_a_repository, &there, &good);
    assert_eq!(code, 2);
    assert_eq!(out, "");
    assert_eq!(
        err,
        "tm: not in a git repository: /repo/src\nnext: Run tm inside a git repository, or pass --rules <path>.\n"
    );
}

#[test]
fn a_missing_rule_file_under_json_is_also_an_object_on_stdout() {
    let (code, out, err) = rules_in_repo(json_args(), &repo_top, &|_: &Path| false, &good);
    assert_eq!(code, 2);
    let value: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(
        value,
        json!({
            "error": "no rule file at /repo/temperare.toml",
            "exit": 2,
            "next": "Write temperare.toml at the top of the repository, or pass --rules <path>.",
        })
    );
    assert_eq!(
        err,
        "tm: no rule file at /repo/temperare.toml\n\
         next: Write temperare.toml at the top of the repository, or pass --rules <path>.\n"
    );
}

#[test]
fn the_rules_flag_reads_that_file_and_never_asks_git() {
    let toplevel = |_: &Path| -> Result<PathBuf, GitError> { panic!("toplevel must not be called") };
    let exists = |_: &Path| -> bool { panic!("exists must not be called") };
    let read = |p: &Path| {
        assert_eq!(p, Path::new("/repo/src/cfg/t.toml"));
        Ok(RuleFile::parse(GOOD, Path::new(".")).unwrap())
    };
    let args = RulesArgs { rules: Some(PathBuf::from("cfg/t.toml")), json: false };
    let (code, out, _) = rules_in_repo(args, &toplevel, &exists, &read);
    assert_eq!(code, 0);
    assert!(out.starts_with("rule file: /repo/src/cfg/t.toml\n"), "{out}");
}

fn clippy_not_available(engine: Engine) -> Availability {
    assert_eq!(engine, Engine::Clippy);
    Availability::NotAvailable { why: "cargo-clippy not found on PATH".to_string() }
}

#[test]
fn text_with_an_engine_not_available_still_exits_0_with_nothing_to_do() {
    let (code, out, err) = rules_with(RulesArgs::default(), &repo_top, &there, &good, &clippy_not_available);
    assert_eq!(code, 0);
    assert_eq!(
        out,
        "rule file: /repo/temperare.toml\n\
         rules (1):\n  \
         lint-all  clippy  quality  error  paths: all files  not available: cargo-clippy not found on PATH\n\
         exceptions in force (1):\n  \
         lint-all  src/legacy.rs  approved: bob 2026-11-17  reason: Legacy.\n\
         next: nothing to do\n"
    );
    assert_eq!(err, "");
}

#[test]
fn json_with_an_engine_not_available_still_exits_0_and_says_why() {
    let (code, out, err) = rules_with(json_args(), &repo_top, &there, &good, &clippy_not_available);
    assert_eq!(code, 0);
    let value: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(value["rules"][0]["available"], json!(false));
    assert_eq!(value["rules"][0]["why"], json!("cargo-clippy not found on PATH"));
    assert_eq!(value["exit"], json!(0));
    assert_eq!(value["next"], json!("nothing to do"));
    assert_eq!(err, "");
}
