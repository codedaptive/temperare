//! `availability`: whether an engine can be run here.

use crate::entry_for::entry_for;
use crate::no_adapter_why::no_adapter_why;
use crate::types::Availability;
use rules::Engine;

/// Whether `engine` can be run here, from a PATH lookup only (nothing is run). Clippy needs both `cargo-clippy` and
/// `clippy-driver`; the others need the program they run as. Tessera is never available: its slop records are not
/// defined yet, so there is nothing to read from `ts` (settled S-20).
pub fn availability(engine: Engine) -> Availability {
    availability_from(engine, |name| procio::which(name).is_some())
}

/// `availability` with the PATH lookup given, so tests need no real PATH, read from the engine's entry: an engine with
/// no entry in this build is not available with `no_adapter_why`; an entry that is never available (`never`,
/// Tessera) is not available with that why and nothing is looked up; otherwise each name the entry `needs` is looked
/// up in order, and the first not found is the reason. Clippy needs `cargo-clippy`, which
/// `cargo clippy` runs, then `clippy-driver`, which its flow runs for `-W help`, so clippy cannot run without it.
pub(crate) fn availability_from(engine: Engine, on_path: impl Fn(&str) -> bool) -> Availability {
    let Some(entry) = entry_for(engine) else {
        return Availability::NotAvailable { why: no_adapter_why(engine) };
    };
    if let Some(why) = entry.never {
        return Availability::NotAvailable { why: why.to_string() };
    }
    match entry.needs.iter().find(|name| !on_path(name)) {
        Some(name) => Availability::NotAvailable { why: format!("{name} not found on PATH") },
        None => Availability::Available,
    }
}
