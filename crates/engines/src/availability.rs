//! `availability`: whether an engine can be run here.

use crate::program::program;
use crate::types::Availability;
use rules::Engine;

/// Whether `engine` can be run here, from a PATH lookup only (nothing is run). Clippy needs both `cargo-clippy` and
/// `clippy-driver`; the others need the program they run as. Tessera is never available: its slop records are not
/// defined yet, so there is nothing to read from `ts` (settled S-20).
pub fn availability(engine: Engine) -> Availability {
    availability_from(engine, |name| procio::which(name).is_some())
}

/// `availability` with the PATH lookup given, so tests need no real PATH. Clippy looks up `cargo-clippy`, which
/// `cargo clippy` runs, then `clippy-driver`, which the adapter runs for `-W help`, so clippy cannot run without it;
/// Tessera looks up nothing; the others look up the program they run as. The first name not found is the reason.
pub(crate) fn availability_from(engine: Engine, on_path: impl Fn(&str) -> bool) -> Availability {
    let names: &[&str] = match engine {
        Engine::Tessera => {
            return Availability::NotAvailable { why: "Tessera's slop records are not defined yet".to_string() }
        }
        Engine::Clippy => &["cargo-clippy", "clippy-driver"],
        other => &[program(other)],
    };
    match names.iter().find(|name| !on_path(name)) {
        Some(name) => Availability::NotAvailable { why: format!("{name} not found on PATH") },
        None => Availability::Available,
    }
}
