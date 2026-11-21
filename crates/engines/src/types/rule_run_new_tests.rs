//! Unit tests for rule_run_new.rs: `new` gives exactly the fields it was given.

use super::RuleRun;
use rules::{Engine, RuleOutcome};

#[test]
fn new_with_a_count_gives_its_fields() {
    let got = RuleRun::new("none-compare".to_string(), Engine::Ruff, RuleOutcome::Findings(Vec::new()), Some(862));
    let want = RuleRun {
        rule: "none-compare".to_string(),
        engine: Engine::Ruff,
        outcome: RuleOutcome::Findings(Vec::new()),
        raw_chars: Some(862),
    };
    assert_eq!(got, want);
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
    let want = RuleRun { rule: "force-cast".to_string(), engine: Engine::SwiftLint, outcome: failed(), raw_chars: None };
    assert_eq!(got, want);
}
