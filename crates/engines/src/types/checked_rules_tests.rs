//! Unit tests for checked_rules.rs: plain data; equality covers the runs and the engines.

use super::{CheckedRules, EngineVersion, RuleRun};
use rules::{Engine, RuleOutcome};

fn checked(raw_chars: Option<usize>, version: Option<&str>) -> CheckedRules {
    CheckedRules {
        runs: vec![RuleRun::new("none-compare".to_string(), Engine::Ruff, RuleOutcome::Findings(Vec::new()), raw_chars)],
        engines: vec![EngineVersion { engine: Engine::Ruff, version: version.map(str::to_string) }],
    }
}

#[test]
fn checked_rules_compare_by_runs_and_engines() {
    assert_eq!(checked(Some(2), Some("0.17.4")), checked(Some(2), Some("0.17.4")).clone());
    assert_ne!(checked(Some(2), Some("0.17.4")), checked(Some(3), Some("0.17.4")));
    assert_ne!(checked(Some(2), Some("0.17.4")), checked(Some(2), None));
}
