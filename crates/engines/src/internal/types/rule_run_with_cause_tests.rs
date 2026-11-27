//! Unit test for setting a rule run's not-available cause.

use super::{NotAvailableCause, RuleRun};
use rules::{Engine, RuleOutcome};

#[test]
fn with_cause_sets_the_cause_and_keeps_every_other_field() {
    let got = RuleRun::new(
        "r".into(),
        Engine::Ruff,
        RuleOutcome::NotAvailable { rule: "r".into(), engine: Engine::Ruff, why: "unknown check".into() },
        Some(77),
    ).with_cause(NotAvailableCause::UnknownCheck);
    assert_eq!(got, RuleRun {
        rule: "r".into(),
        engine: Engine::Ruff,
        outcome: RuleOutcome::NotAvailable { rule: "r".into(), engine: Engine::Ruff, why: "unknown check".into() },
        raw_chars: Some(77),
        no_files: None,
        not_available_cause: Some(NotAvailableCause::UnknownCheck),
    });
}
