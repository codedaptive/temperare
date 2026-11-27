//! Unit test for not_available_cause.rs: a not-available `RuleRun` keeps each of the five causes it is given, so a caller
//! can tell `EngineNotOnPath`, `HelperNotOnPath`, `UnknownCheck`, `NoCargoProject` and `Other` apart on the run.

use super::{NotAvailableCause, RuleRun};
use rules::{Engine, RuleOutcome};

#[test]
fn not_available_cause_each_cause_is_kept_on_the_run() {
    let rows = [
        (NotAvailableCause::EngineNotOnPath, Some(NotAvailableCause::EngineNotOnPath)),
        (NotAvailableCause::HelperNotOnPath, Some(NotAvailableCause::HelperNotOnPath)),
        (NotAvailableCause::UnknownCheck, Some(NotAvailableCause::UnknownCheck)),
        (NotAvailableCause::NoCargoProject, Some(NotAvailableCause::NoCargoProject)),
        (NotAvailableCause::Other, Some(NotAvailableCause::Other)),
    ];
    let wrong: Vec<_> = rows
        .into_iter()
        .map(|(cause, want)| {
            let got = RuleRun::new(
                "r".into(),
                Engine::Ruff,
                RuleOutcome::NotAvailable { rule: "r".into(), engine: Engine::Ruff, why: "not on PATH".into() },
                None,
            )
            .with_cause(cause)
            .not_available_cause;
            (cause, got, want)
        })
        .filter(|(_, got, want)| got != want)
        .collect();
    assert!(wrong.is_empty(), "rows as (cause, got, want): {wrong:?}");
}
