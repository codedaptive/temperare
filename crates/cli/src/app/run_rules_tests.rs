//! ZUT for app/run_rules.rs: the listing (with its not-applicable rules), the problems and the discovery errors, as text
//! and as JSON, with git, the disk, engine availability, the no-file rules and the clock as closures and rule files
//! parsed from literal TOML (one test builds its rule file in code: `baseline_and_own`).

use super::{run_rules, RulesWorld};
use crate::RulesArgs;
use engines::{Availability, NoFileRule};
use procio::{GitError, Oid};
use rules::{Engine, Problem, Rule, RuleFile};
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
expires = "2099-12-31"
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

/// 2026-11-21 12:00:00 UTC: 20778 days (56 * 365 + 14 leap days to 2026-01-01, then 324 days to 2026-11-21) *
/// 86400 + 43200 = 1_795_219_200 + 43_200. GOOD's exception expires long after it.
fn pinned_now() -> u64 {
    1_795_262_400
}

/// Runs `tm rules` in /repo/src with the given seams, every engine available; (exit, out, err).
fn rules_in_repo(
    args: RulesArgs,
    toplevel: &dyn Fn(&Path) -> Result<PathBuf, GitError>,
    exists: &dyn Fn(&Path) -> bool,
    read: &dyn Fn(&Path) -> Result<RuleFile, Vec<Problem>>,
) -> (u8, String, String) {
    rules_with(args, toplevel, exists, read, &all_available, &pinned_now)
}

/// No rule settled as no-file. It must be asked about the rule file read (every good one here holds only `lint-all`)
/// in the folder tm runs in, /repo/src, not the top of the repository.
fn none_no_file(rf: &RuleFile, folder: &Path) -> Vec<NoFileRule> {
    assert_eq!(rf.rules.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(), ["lint-all"]);
    assert_eq!(folder, Path::new("/repo/src"));
    Vec::new()
}

/// Runs `tm rules` in /repo/src with the given seams, engine availability and clock, and no rule settled as no-file;
/// (exit, out, err).
fn rules_with(
    args: RulesArgs,
    toplevel: &dyn Fn(&Path) -> Result<PathBuf, GitError>,
    exists: &dyn Fn(&Path) -> bool,
    read: &dyn Fn(&Path) -> Result<RuleFile, Vec<Problem>>,
    availability: &dyn Fn(Engine) -> Availability,
    now: &dyn Fn() -> u64,
) -> (u8, String, String) {
    rules_in_world(args, toplevel, exists, read, availability, now, &none_no_file)
}

/// `rules_with` with the no-file seam given too. `rev_parse` and `blob_at` panic: `tm rules` without `--loosened`
/// never asks git for a base.
fn rules_in_world(
    args: RulesArgs,
    toplevel: &dyn Fn(&Path) -> Result<PathBuf, GitError>,
    exists: &dyn Fn(&Path) -> bool,
    read: &dyn Fn(&Path) -> Result<RuleFile, Vec<Problem>>,
    availability: &dyn Fn(Engine) -> Availability,
    now: &dyn Fn() -> u64,
    no_file_rules: &dyn Fn(&RuleFile, &Path) -> Vec<NoFileRule>,
) -> (u8, String, String) {
    let rev_parse = |_: &Path, _: &str| -> Result<Option<Oid>, GitError> { panic!("rev_parse must not be called") };
    let blob_at =
        |_: &Path, _: &str, _: &Path| -> Result<Option<Vec<u8>>, GitError> { panic!("blob_at must not be called") };
    let world =
        RulesWorld { toplevel, exists, read, availability, now, rev_parse: &rev_parse, blob_at: &blob_at, no_file_rules };
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = run_rules(&args, Path::new("/repo/src"), &world, &mut out, &mut err);
    (code, String::from_utf8(out).unwrap(), String::from_utf8(err).unwrap())
}

fn json_args() -> RulesArgs {
    RulesArgs { json: true, ..RulesArgs::default() }
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
         not applicable (0):\n\
         exceptions in force (1):\n  \
         lint-all  src/legacy.rs  approved: bob 2026-11-17  reason: Legacy.\n\
         exceptions expired (0):\n\
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
            "not_applicable": [],
            "rules_not_applicable": 0,
            "exceptions": [{
                "rule": "lint-all", "path": "src/legacy.rs", "reason": "Legacy.",
                "approved": "bob 2026-11-17",
            }],
            "exceptions_in_force": 1,
            "expired": [],
            "exceptions_expired": 0,
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
    let args = RulesArgs { rules: Some(PathBuf::from("cfg/t.toml")), ..RulesArgs::default() };
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
    let (code, out, err) = rules_with(RulesArgs::default(), &repo_top, &there, &good, &clippy_not_available, &pinned_now);
    assert_eq!(code, 0);
    assert_eq!(
        out,
        "rule file: /repo/temperare.toml\n\
         rules (1):\n  \
         lint-all  clippy  quality  error  paths: all files  not available: cargo-clippy not found on PATH\n\
         not applicable (0):\n\
         exceptions in force (1):\n  \
         lint-all  src/legacy.rs  approved: bob 2026-11-17  reason: Legacy.\n\
         exceptions expired (0):\n\
         next: nothing to do\n"
    );
    assert_eq!(err, "");
}

#[test]
fn json_with_an_engine_not_available_still_exits_0_and_says_why() {
    let (code, out, err) = rules_with(json_args(), &repo_top, &there, &good, &clippy_not_available, &pinned_now);
    assert_eq!(code, 0);
    let value: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(value["rules"][0]["available"], json!(false));
    assert_eq!(value["rules"][0]["why"], json!("cargo-clippy not found on PATH"));
    assert_eq!(value["exit"], json!(0));
    assert_eq!(value["next"], json!("nothing to do"));
    assert_eq!(err, "");
}

const DATES: &str = r#"version = 1

[[rule]]
id = "lint-all"
engine = "clippy"
check = "clippy::all"
fix = "Do what clippy says."

[[exception]]
rule = "lint-all"
path = "src/d.rs"
reason = "Ends on D."
approved = "bob 2026-11-01"
expires = "2026-11-30"

[[exception]]
rule = "lint-all"
path = "src/d1.rs"
reason = "Ends the day before D."
approved = "bob 2026-11-01"
expires = "2026-11-29"

[[exception]]
rule = "lint-all"
path = "src/far.rs"
reason = "Ends far later."
approved = "bob 2026-11-01"
expires = "2099-12-31"
"#;

#[test]
fn an_exception_holds_through_its_expires_day_in_utc_and_is_listed_expired_from_the_day_after() {
    let read = |_: &Path| Ok(RuleFile::parse(DATES, Path::new(".")).unwrap());
    let d = json!({"rule": "lint-all", "path": "src/d.rs", "reason": "Ends on D.", "approved": "bob 2026-11-01"});
    let far =
        json!({"rule": "lint-all", "path": "src/far.rs", "reason": "Ends far later.", "approved": "bob 2026-11-01"});
    let d_expired = json!({"rule": "lint-all", "path": "src/d.rs", "reason": "Ends on D.",
                           "approved": "bob 2026-11-01", "expires": "2026-11-30"});
    let d1_expired = json!({"rule": "lint-all", "path": "src/d1.rs", "reason": "Ends the day before D.",
                            "approved": "bob 2026-11-01", "expires": "2026-11-29"});
    // (row, now, text exception lines, json exceptions, json expired)
    let rows = [
        (
            // D = 2026-11-30 at 23:59:59 UTC: 20787 days (56 * 365 + 14 leap days to 2026-01-01, then 333 days to
            // 2026-11-30) * 86400 + 86399 = 1_795_996_800 + 86_399.
            "on D, its last second",
            1_796_083_199_u64,
            "exceptions in force (2):\n  \
             lint-all  src/d.rs  approved: bob 2026-11-01  reason: Ends on D.\n  \
             lint-all  src/far.rs  approved: bob 2026-11-01  reason: Ends far later.\n\
             exceptions expired (1):\n  \
             lint-all  src/d1.rs  expires: 2026-11-29  approved: bob 2026-11-01  reason: Ends the day before D.\n",
            json!([d, far]),
            json!([d1_expired]),
        ),
        (
            // D+1 = 2026-12-01 at 00:00:01 UTC: 20788 days * 86400 + 1 = 1_796_083_200 + 1.
            "on D+1, its second second",
            1_796_083_201_u64,
            "exceptions in force (1):\n  \
             lint-all  src/far.rs  approved: bob 2026-11-01  reason: Ends far later.\n\
             exceptions expired (2):\n  \
             lint-all  src/d.rs  expires: 2026-11-30  approved: bob 2026-11-01  reason: Ends on D.\n  \
             lint-all  src/d1.rs  expires: 2026-11-29  approved: bob 2026-11-01  reason: Ends the day before D.\n",
            json!([far]),
            json!([d_expired, d1_expired]),
        ),
    ];
    for (row, now, lines, in_force, expired) in rows {
        let clock = || now;
        let (code, out, _) = rules_with(RulesArgs::default(), &repo_top, &there, &read, &all_available, &clock);
        assert_eq!(code, 0, "{row}");
        let expected = format!(
            "rule file: /repo/temperare.toml\n\
             rules (1):\n  \
             lint-all  clippy  quality  error  paths: all files  available\n\
             not applicable (0):\n\
             {lines}next: nothing to do\n"
        );
        assert_eq!(out, expected, "{row}");
        let (code, out, _) = rules_with(json_args(), &repo_top, &there, &read, &all_available, &clock);
        assert_eq!(code, 0, "{row}");
        let value: serde_json::Value = serde_json::from_str(&out).unwrap();
        assert_eq!(value["exceptions_in_force"], json!(in_force.as_array().unwrap().len()), "{row}");
        assert_eq!(value["exceptions"], in_force, "{row}");
        assert_eq!(value["exceptions_expired"], json!(expired.as_array().unwrap().len()), "{row}");
        assert_eq!(value["expired"], expired, "{row}");
    }
}

/// A rule file built in code: `tm-x` and the repository's own `own`, both clippy, in that order.
fn baseline_and_own(_: &Path) -> Result<RuleFile, Vec<Problem>> {
    let tm_x = Rule::new("tm-x", Engine::Clippy, "check");
    let own = Rule { position: 1, ..Rule::new("own", Engine::Clippy, "check") };
    Ok(RuleFile { rules: vec![tm_x, own], ..RuleFile::new(PathBuf::from(".")) })
}

#[test]
fn the_rules_the_no_file_seam_settles_in_cwd_are_listed_not_applicable_with_its_why() {
    // Kills: the seam not called, its list dropped (`&[]` passed on), the seam asked about the top (/repo) not cwd.
    let flags = |rf: &RuleFile, folder: &Path| -> Vec<NoFileRule> {
        assert_eq!(rf.rules.iter().map(|r| r.id.as_str()).collect::<Vec<_>>(), ["tm-x", "own"]);
        assert_eq!(folder, Path::new("/repo/src"));
        ["tm-x", "own"]
            .map(|id| NoFileRule { rule: id.to_string(), engine: Engine::Clippy, why: "no file here".to_string() })
            .to_vec()
    };
    let run = |args| rules_in_world(args, &repo_top, &there, &baseline_and_own, &all_available, &pinned_now, &flags);
    let (code, out, _) = run(RulesArgs::default());
    assert_eq!(code, 0);
    assert_eq!(
        out,
        "rule file: /repo/temperare.toml\n\
         rules (2):\n  \
         tm-x  clippy  quality  error  paths: all files  available\n  \
         own  clippy  quality  error  paths: all files  available\n\
         not applicable (1):\n  \
         tm-x  clippy  no file here\n\
         exceptions in force (0):\n\
         exceptions expired (0):\n\
         next: nothing to do\n"
    );
    let (code, out, _) = run(json_args());
    assert_eq!(code, 0);
    let value: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(value["not_applicable"], json!([{"rule": "tm-x", "engine": "clippy", "why": "no file here"}]));
    assert_eq!(value["rules_not_applicable"], json!(1));
}
