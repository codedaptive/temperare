//! Unit test for rule_run_new.rs: `new` gives exactly the fields it was given, and no `no_files` flag.

use super::RuleRun;
use rules::{Engine, RuleOutcome};

#[test]
fn new_with_a_count_gives_its_fields() {
    let got = RuleRun::new("none-compare".to_string(), Engine::Ruff, RuleOutcome::Findings(Vec::new()), Some(862));
    assert_eq!(got.rule, "none-compare");
    assert_eq!(got.engine, Engine::Ruff);
    assert_eq!(got.outcome, RuleOutcome::Findings(Vec::new()));
    assert_eq!(got.raw_chars, Some(862));
}

#[test]
fn new_with_no_count_gives_its_fields() {
    let failed = || RuleOutcome::Failed {
        rule: "force-cast".to_string(),
        engine: Engine::SwiftLint,
        code: Some(2),
        stderr: "boom".to_string(),
    };
    let got = RuleRun::new("force-cast".to_string(), Engine::SwiftLint, failed(), None);
    assert_eq!(got.rule, "force-cast");
    assert_eq!(got.engine, Engine::SwiftLint);
    assert_eq!(got.outcome, failed());
    assert_eq!(got.raw_chars, None);
}

#[test]
fn new_has_no_no_files_flag_even_for_an_empty_run_with_no_count() {
    let got = RuleRun::new("quote".to_string(), Engine::ShellCheck, RuleOutcome::Findings(Vec::new()), None);
    assert_eq!(got.no_files, None);
}
