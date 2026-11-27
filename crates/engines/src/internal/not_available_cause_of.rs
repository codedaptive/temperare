//! `not_available_cause_of`: the not-available cause a rule's stop gives its run.

use crate::entry_of::entry_of;
use crate::types::{NotAvailableCause, Stop};
use rules::Engine;

/// The cause of the `NotAvailable` outcome `stopped` reads `stop` as for a rule on `engine`, None for a stop it reads
/// otherwise. A program not on PATH is `EngineNotOnPath` when the engine's entry `needs` it (the engine's own
/// programs, such as clippy's `cargo-clippy` and `clippy-driver`, and a Node engine's `node`), else `HelperNotOnPath`
/// (such as `cargo` under clippy, or git run to list ShellCheck's scripts); a check the engine does not know is
/// `UnknownCheck`; a clippy rule with no `Cargo.toml` at or above its folder is `NoCargoProject`, by the stop's variant
/// and never its why text; any other not-available stop (`NotAvailable`, `NotListed`) is `Other`; `Failed` and
/// `NoFiles` are not not-available.
pub(crate) fn not_available_cause_of(stop: &Stop, engine: Engine) -> Option<NotAvailableCause> {
    match stop {
        Stop::NotOnPath { program } if entry_of(engine).needs.contains(&program.as_str()) => {
            Some(NotAvailableCause::EngineNotOnPath)
        }
        Stop::NotOnPath { .. } => Some(NotAvailableCause::HelperNotOnPath),
        Stop::UnknownCheck { .. } => Some(NotAvailableCause::UnknownCheck),
        Stop::NoCargoProject { .. } => Some(NotAvailableCause::NoCargoProject),
        Stop::NotAvailable { .. } | Stop::NotListed { .. } => Some(NotAvailableCause::Other),
        Stop::Failed { .. } | Stop::NoFiles { .. } => None,
    }
}
