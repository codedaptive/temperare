//! `run_rules_versioned`: every rule of a rule file run by its engine, with no base, with the version of each engine
//! named.

use crate::run_rules_versioned_at::run_rules_versioned_at;
use crate::types::CheckedRules;
use rules::{RelPath, RuleFile};
use std::path::Path;

/// The check with no base: `run_rules_versioned_at` with `None`, whose doc gives what the result holds.
pub fn run_rules_versioned(file: &RuleFile, repo: &Path, only: &[RelPath]) -> CheckedRules {
    run_rules_versioned_at(file, repo, only, None)
}
