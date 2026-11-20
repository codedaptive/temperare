//! `no_script`: ShellCheck's no-file outcome, when no listed script is one its rule applies to.

use crate::types::Stop;
use rules::{RelPath, Rule};
use std::path::PathBuf;

/// Why a rule ran nothing when tm hands its engine the files and none is one the engine checks.
const NO_ENGINE_FILE_WHY: &str = "no file its engine checks";

/// `NoFiles` with `NO_ENGINE_FILE_WHY` when none of `scripts` (relative to the rule's folder) is one `rule` applies
/// to, so a rule whose paths hold no script reads as having nothing to check, never as clean; `None` otherwise.
pub(crate) fn no_script(rule: &Rule, scripts: &[PathBuf]) -> Option<Stop> {
    if scripts.iter().any(|script| rule.applies(&RelPath::new(&script.to_string_lossy()))) {
        return None;
    }
    Some(Stop::NoFiles { why: NO_ENGINE_FILE_WHY.to_string() })
}
