//! `run_rules_counted`: every rule of a rule file run by its engine, each with its outcome and the characters its
//! check run printed.

use crate::host::Host;
use crate::run_rules_counted_with::run_rules_counted_with;
use crate::types::RuleRun;
use rules::{RelPath, RuleFile};
use std::cell::Cell;
use std::path::Path;

/// Each rule's run, in rule-file order. The rule file's folder is `file.folder`, relative to `repo` unless absolute;
/// every finding's path is relative to it. `only` empty checks the whole folder; otherwise findings are cut to the
/// files and folders it lists, a folder keeping what lies under it (engines still run over the folder). An absent
/// engine is `NotAvailable` with `availability`'s why; an engine that ran but gave no answer is `Failed`; otherwise
/// `Findings`, empty when clean. Each run's `raw_chars` is what its engine's check run printed, `None` when that run
/// did not run: summed over the rules it is the raw output a check read, and its `Some` values count the rules whose
/// engine ran.
pub fn run_rules_counted(file: &RuleFile, repo: &Path, only: &[RelPath]) -> Vec<RuleRun> {
    let host = Host {
        run: &procio::run,
        on_path: &|program: &str| procio::which(program).is_some(),
        raw_chars: &Cell::new(None),
    };
    run_rules_counted_with(file, repo, only, &host)
}
