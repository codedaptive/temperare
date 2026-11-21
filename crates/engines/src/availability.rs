//! `availability`: whether an engine can be run here.

use crate::not_on_path_why::not_on_path_why;
use crate::types::{Availability, Stop};
use crate::unavailable::unavailable;
use rules::Engine;

/// Whether `engine` can be run here, from a PATH lookup only (nothing is run). Clippy needs both `cargo-clippy` and
/// `clippy-driver`; the others need the program they run as. Tessera is never available: its slop records are not
/// defined yet, so there is nothing to read from `ts`.
pub fn availability(engine: Engine) -> Availability {
    availability_from(engine, |name| procio::which(name).is_some())
}

/// `availability` with the PATH lookup given, so tests need no real PATH: `unavailable`'s stop read as
/// `NotAvailable` with its why (`not_on_path_why` for a missing name), whether the engine has no entry in this build
/// (`no_adapter_why`), is never available (`never`, Tessera, with nothing looked up) or needs a name not on PATH (each
/// name the entry `needs` looked up in order, the first not found the reason). Clippy needs `cargo-clippy`, which
/// `cargo clippy` runs, then `clippy-driver`, which its flow runs for `-W help`, so clippy cannot run without it.
pub(crate) fn availability_from(engine: Engine, on_path: impl Fn(&str) -> bool) -> Availability {
    match unavailable(engine, on_path) {
        Some(Stop::NotAvailable { why }) => Availability::NotAvailable { why },
        Some(Stop::NotOnPath { program }) => Availability::NotAvailable { why: not_on_path_why(&program) },
        _ => Availability::Available,
    }
}
