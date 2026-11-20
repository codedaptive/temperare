//! ZUT for rule_run.rs: plain data; equality covers every field, and a run that printed nothing is not one that
//! did not run.

use super::RuleRun;
use rules::{Engine, RuleOutcome};

fn run(raw_chars: Option<usize>) -> RuleRun {
    RuleRun {
        rule: "none-compare".to_string(),
        engine: Engine::Ruff,
        outcome: RuleOutcome::Findings(Vec::new()),
        raw_chars,
    }
}

#[test]
fn rule_runs_compare_by_every_field() {
    assert_eq!(run(Some(12)), run(Some(12)).clone());
    assert_ne!(run(Some(12)), run(Some(13)));
    assert_ne!(run(Some(0)), run(None));
    assert_ne!(run(None), RuleRun { rule: "other".to_string(), ..run(None) });
    assert_ne!(run(None), RuleRun { engine: Engine::SwiftLint, ..run(None) });
    let failed =
        RuleOutcome::Failed { rule: "none-compare".to_string(), engine: Engine::Ruff, code: None, stderr: "x".to_string() };
    assert_ne!(run(None), RuleRun { outcome: failed, ..run(None) });
}
