//! `availability`: whether an engine can be run here.

use crate::program::program;
use crate::types::Availability;
use rules::Engine;

/// Whether `engine` can be run here, from a PATH lookup only (nothing is run). Tessera is never available: its slop
/// records are not defined yet, so there is nothing to read from `ts` (settled S-20).
pub fn availability(engine: Engine) -> Availability {
    availability_from(engine, |name| procio::which(name).is_some())
}

/// `availability` with the PATH lookup given, so tests need no real PATH. Clippy looks for `cargo-clippy`, which
/// `cargo clippy` runs; Tessera looks up nothing; the others look for the program they run as.
pub(crate) fn availability_from(engine: Engine, on_path: impl Fn(&str) -> bool) -> Availability {
    let name = match engine {
        Engine::Tessera => {
            return Availability::NotAvailable { why: "Tessera's slop records are not defined yet".to_string() }
        }
        Engine::Clippy => "cargo-clippy",
        other => program(other),
    };
    if on_path(name) {
        return Availability::Available;
    }
    Availability::NotAvailable { why: format!("{name} not found on PATH") }
}
