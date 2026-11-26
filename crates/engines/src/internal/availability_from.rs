//! `availability_from`: whether an engine can be run here, with the PATH lookup given.

use crate::not_on_path_why::not_on_path_why;
use crate::types::{Availability, Stop};
use crate::unavailable::unavailable;
use rules::Engine;
use std::path::Path;

/// Whether `engine` can be run for the rule file's absolute canonical `folder` (`None` when there is no folder, so a
/// local ESLint in a folder's `node_modules/.bin` is not seen), with the PATH lookup given, so tests need no real
/// PATH: `unavailable`'s stop read as `NotAvailable` with its why (`not_on_path_why` for a missing name), whether the
/// engine is never available (`never`, Tessera, with nothing looked up) or needs a name not found (each name the entry
/// `needs` checked in order, the first not found the reason; ESLint's program counts as found when `program_for` finds
/// an executable local one). Clippy needs `cargo-clippy`, which `cargo clippy` runs, then `clippy-driver`, which its
/// flow runs for `-W help`, so clippy cannot run without it. The Node engines need their program and `node`; the
/// others need the program they run as. Tessera is never available: its slop records are not defined yet, so there
/// is nothing to read from `ts` (settled S-20).
pub(crate) fn availability_from(engine: Engine, folder: Option<&Path>, on_path: impl Fn(&str) -> bool) -> Availability {
    match unavailable(engine, folder, on_path) {
        Some(Stop::NotAvailable { why }) => Availability::NotAvailable { why },
        Some(Stop::NotOnPath { program }) => Availability::NotAvailable { why: not_on_path_why(&program) },
        _ => Availability::Available,
    }
}
