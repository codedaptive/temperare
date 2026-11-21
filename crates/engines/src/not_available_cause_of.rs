//! `not_available_cause_of`: the not-available cause a rule's stop gives its run.

use crate::entry_for::entry_for;
use crate::types::{NotAvailableCause, Stop};
use rules::Engine;

/// The cause of the `NotAvailable` outcome `stopped` reads `stop` as for a rule on `engine`, None for a stop it reads
/// otherwise. A program not on PATH is `EngineNotOnPath` when the engine's entry `needs` it (the programs installing
/// the engine supplies, such as clippy's `cargo-clippy` and `clippy-driver`), else `HelperNotOnPath` (such as `cargo`
/// under clippy, or git run to list ShellCheck's scripts); a check the engine does not know is `UnknownCheck`; any
/// other not-available stop (`NotAvailable`, `NotListed`) is `Other`; `Failed` and `NoFiles` are not not-available.
pub(crate) fn not_available_cause_of(stop: &Stop, engine: Engine) -> Option<NotAvailableCause> {
    match stop {
        Stop::NotOnPath { program } if entry_for(engine).is_some_and(|entry| entry.needs.contains(&program.as_str())) => {
            Some(NotAvailableCause::EngineNotOnPath)
        }
        Stop::NotOnPath { .. } => Some(NotAvailableCause::HelperNotOnPath),
        Stop::UnknownCheck { .. } => Some(NotAvailableCause::UnknownCheck),
        Stop::NotAvailable { .. } | Stop::NotListed { .. } => Some(NotAvailableCause::Other),
        Stop::Failed { .. } | Stop::NoFiles { .. } => None,
    }
}
