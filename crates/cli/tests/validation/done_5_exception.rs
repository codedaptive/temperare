//! Done 5: an exception. "An approved exception in the rule file silences its finding and is counted in the report."
//! Every report counts the exceptions in force (mission 4), with or without a finding to silence.

use crate::check::Check;
use crate::repo::Repo;
use crate::cctao::{cctao, Answer};

const RULES: &str = r#"version = 1

[[rule]]
id = "no-none-compare"
engine = "ruff"
check = "E711"
severity = "error"
fix = "Compare with `is None`."

[[exception]]
rule = "no-none-compare"
path = "legacy.py"
reason = "the legacy module is rewritten in 2.0"
approved = "reviewer 2026-11-17"
expires = "2099-12-31"
"#;

/// `cctao check --json` over a repository holding `legacy` as legacy.py, recorded in `check`.
fn check_legacy(check: &mut Check, legacy: &str) -> Answer {
    let repo = Repo::new();
    repo.write("tao.toml", RULES);
    repo.write("legacy.py", legacy);
    repo.commit("legacy module");
    let args = ["check", "--json"];
    let a = cctao(&repo, &args);
    check.saw_cctao(&args, a.code, &a.json);
    a
}

#[test]
fn an_approved_exception_silences_its_finding_and_is_counted() {
    let mut check = Check::new(
        "an_approved_exception_silences_its_finding_and_is_counted",
        "rule no-none-compare (ruff, E711) with an approved exception for legacy.py, which has `x == None` at line 2",
    );
    let a = check_legacy(&mut check, "def old(x):\n    return x == None\n");
    assert_eq!(a.code, 0, "the only finding is silenced: {:#}\n{}", a.json, a.stderr);
    assert!(a.findings().is_empty(), "nothing kept: {:#}", a.json);
    let silenced = a.json["silenced"].as_array().unwrap_or_else(|| panic!("no silenced list: {:#}", a.json));
    assert_eq!(silenced.len(), 1, "the finding is silenced, not lost: {:#}", a.json);
    assert_eq!(a.json["exceptions_in_force"], 1, "counted: {:#}", a.json);
}

#[test]
fn the_exceptions_in_force_are_counted_with_nothing_to_silence() {
    let mut check = Check::new(
        "the_exceptions_in_force_are_counted_with_nothing_to_silence",
        "rule no-none-compare (ruff, E711) with an approved exception for legacy.py, which uses `x is None`: \
         nothing to silence",
    );
    let a = check_legacy(&mut check, "def old(x):\n    return x is None\n");
    assert_eq!(a.code, 0, "clean: {:#}", a.json);
    assert_eq!(a.json["exceptions_in_force"], 1, "always counted (mission 4): {:#}", a.json);
}
