//! `run_rules_versioned`: every rule of a rule file run by its engine, with the version of each engine named.

use crate::host::Host;
use crate::run_rules_versioned_with::run_rules_versioned_with;
use crate::types::CheckedRules;
use rules::{RelPath, RuleFile};
use std::cell::Cell;
use std::path::Path;

/// One check of a rule file: `runs` is `run_rules_counted`'s, each rule's run in rule-file order; `engines` is each
/// distinct engine the rules name, once, in first-appearance order, with the version its probe read (`None` when the
/// engine is not available, has no probe, the folder cannot be resolved, or the probe gave none). Each available
/// engine is probed once per check, in the rule file's folder when it resolves, before the rules run; a probe is not a
/// check run, so no run's `raw_chars` counts it. A check the engine said it does not know names the engine's version
/// in its why. cli reads `runs` through `run_rules_counted` for the outcomes and counts; `engines` is meant for an
/// `engines: [{engine, version}]` field the report does not have yet, and nothing in report or cli reads it.
pub fn run_rules_versioned(file: &RuleFile, repo: &Path, only: &[RelPath]) -> CheckedRules {
    let host = Host {
        run: &procio::run,
        on_path: &|program: &str| procio::which(program).is_some(),
        raw_chars: &Cell::new(None),
    };
    run_rules_versioned_with(file, repo, only, &host)
}
