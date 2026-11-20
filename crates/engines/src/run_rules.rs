//! `run_rules`: every rule of a rule file run by its engine, each with its outcome.

use crate::run_rules_counted::run_rules_counted;
use rules::{RelPath, RuleFile, RuleOutcome};
use std::path::Path;

/// Each rule's id and outcome, in rule-file order: `run_rules_counted` without the counts. The rule file's folder is
/// `file.folder`, relative to `repo` unless absolute; every finding's path is relative to it. `only` empty checks the
/// whole folder; otherwise findings are cut to the files and folders it lists, a folder keeping what lies under it
/// (engines still run over the folder). An absent engine is `NotAvailable` with `availability`'s why; an engine that
/// ran but gave no answer is `Failed`; otherwise `Findings`, empty when clean.
pub fn run_rules(file: &RuleFile, repo: &Path, only: &[RelPath]) -> Vec<(String, RuleOutcome)> {
    run_rules_counted(file, repo, only).into_iter().map(|run| (run.rule, run.outcome)).collect()
}
