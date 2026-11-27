//! Unit test for rule_run_with_no_files.rs: the flag is set to the reason given and nothing else changes.

use super::RuleRun;
use rules::{Engine, RuleOutcome};

#[test]
fn with_no_files_sets_the_reason_and_keeps_every_other_field() {
    let got = RuleRun::new("quote".to_string(), Engine::ShellCheck, RuleOutcome::Findings(Vec::new()), None)
        .with_no_files("no file its engine checks");
    assert_eq!(got.rule, "quote");
    assert_eq!(got.engine, Engine::ShellCheck);
    assert_eq!(got.outcome, RuleOutcome::Findings(Vec::new()));
    assert_eq!(got.raw_chars, None);
    assert_eq!(got.no_files, Some("no file its engine checks".to_string()));
}

#[test]
fn with_no_files_takes_an_owned_reason_and_replaces_an_earlier_one() {
    let got = RuleRun::new("r".to_string(), Engine::Ruff, RuleOutcome::Findings(Vec::new()), Some(3))
        .with_no_files("first")
        .with_no_files(String::from("its paths match no file in the rule file's folder"));
    assert_eq!(got.raw_chars, Some(3));
    assert_eq!(got.no_files.as_deref(), Some("its paths match no file in the rule file's folder"));
}
