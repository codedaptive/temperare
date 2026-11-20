//! `run_rules_counted`: every rule of a rule file run by its engine, each with its outcome and the characters its
//! check run printed.

use crate::run_rules_versioned::run_rules_versioned;
use crate::types::RuleRun;
use rules::{RelPath, RuleFile};
use std::path::Path;

/// Each rule's run, in rule-file order: `run_rules_versioned`'s `runs`. The rule file's folder is `file.folder`,
/// relative to `repo` unless absolute; every finding's path is relative to it. `only` empty checks the whole folder;
/// otherwise findings are cut to the files and folders it lists, a folder keeping what lies under it (engines still
/// run over the folder). An absent engine is `NotAvailable` with `availability`'s why; a check the engine does not
/// know (an engine too old for it: its own words, or for clippy tm's words for a name its help does not list) is `NotAvailable` with a why naming the engine, its version and the
/// check; an engine that ran but gave no answer is `Failed`; otherwise `Findings`, empty when clean. Each run's
/// `raw_chars` is what its engine's check run printed, `None` when that run did not run: summed over the rules it is
/// the raw output a check read, and its `Some` values count the rules whose engine ran. The version probes are not
/// counted. report and cli read each run's `rule`, `engine`, `outcome` and `raw_chars`.
pub fn run_rules_counted(file: &RuleFile, repo: &Path, only: &[RelPath]) -> Vec<RuleRun> {
    run_rules_versioned(file, repo, only).runs
}
