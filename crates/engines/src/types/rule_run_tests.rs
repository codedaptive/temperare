//! ZUT for rule_run.rs: plain data; equality covers every field, a run that printed nothing is not one that did not
//! run, and a run flagged for having no files is not a plain clean one.

use super::RuleRun;
use rules::{Engine, RuleOutcome};

fn run(raw_chars: Option<usize>) -> RuleRun {
    RuleRun::new("none-compare".to_string(), Engine::Ruff, RuleOutcome::Findings(Vec::new()), raw_chars)
}

#[test]
fn rule_runs_compare_by_every_field() {
    assert_eq!(run(Some(12)), run(Some(12)).clone());
    assert_ne!(run(Some(12)), run(Some(13)));
    assert_ne!(run(Some(0)), run(None));
    assert_ne!(run(None), RuleRun::new("other".to_string(), Engine::Ruff, RuleOutcome::Findings(Vec::new()), None));
    assert_ne!(run(None), RuleRun::new("none-compare".to_string(), Engine::SwiftLint, RuleOutcome::Findings(Vec::new()), None));
    let failed =
        RuleOutcome::Failed { rule: "none-compare".to_string(), engine: Engine::Ruff, code: None, stderr: "x".to_string() };
    assert_ne!(run(None), RuleRun::new("none-compare".to_string(), Engine::Ruff, failed, None));
}

#[test]
fn a_run_flagged_for_no_files_is_not_a_plain_clean_run_and_compares_by_its_reason() {
    let flagged = |why: &str| run(None).with_no_files(why);
    assert_ne!(flagged("no file its engine checks"), run(None));
    assert_eq!(flagged("no file its engine checks"), flagged("no file its engine checks"));
    assert_ne!(flagged("no file its engine checks"), flagged("its paths match no file in the rule file's folder"));
}
