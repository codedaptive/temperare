//! `availability`: whether an engine can be run here.

use crate::not_on_path_why::not_on_path_why;
use crate::types::{Availability, Stop};
use crate::unavailable::unavailable;
use rules::Engine;
use std::path::Path;

/// Whether `engine` can be run here, with no rule file's folder: the PATH lookup only (nothing is run), so a local
/// ESLint in a folder's `node_modules/.bin` is not seen. It is kept for cli, which calls it through a `Fn(Engine)`
/// seam with no folder, until cli moves to `availability_in`, which answers for a rule file's folder with the program
/// the run uses. Clippy needs both `cargo-clippy` and `clippy-driver`; the Node engines need their program and
/// `node`; the others need the program they run as. Tessera is never available: its slop records are not defined
/// yet, so there is nothing to read from `ts` (settled S-20).
pub fn availability(engine: Engine) -> Availability {
    availability_from(engine, None, |name| procio::which(name).is_some())
}

/// `availability` for the rule file's canonical `folder`, with the PATH lookup given, so tests need no real PATH:
/// `unavailable`'s stop read as `NotAvailable` with its why (`not_on_path_why` for a missing name), whether the engine
/// is never available (`never`, Tessera, with nothing looked up) or needs a name not found (each name the entry
/// `needs` checked in order, the first not found the reason; ESLint's program counts as found when `folder` holds a
/// local one, `program_for`). Clippy needs `cargo-clippy`, which `cargo clippy` runs, then `clippy-driver`, which its
/// flow runs for `-W help`, so clippy cannot run without it.
pub(crate) fn availability_from(engine: Engine, folder: Option<&Path>, on_path: impl Fn(&str) -> bool) -> Availability {
    match unavailable(engine, folder, on_path) {
        Some(Stop::NotAvailable { why }) => Availability::NotAvailable { why },
        Some(Stop::NotOnPath { program }) => Availability::NotAvailable { why: not_on_path_why(&program) },
        _ => Availability::Available,
    }
}
