//! `unavailable`: why an engine cannot be run here, as the `Stop` its rules get.

use crate::entry_of::entry_of;
use crate::program_for::program_for;
use crate::types::Stop;
use rules::Engine;
use std::path::Path;

/// The `Stop` of a rule on `engine` when the engine cannot be run here, read from the engine's entry (`entry_of`);
/// None when it can be run. Nothing is run. An entry that is never available (`never`, Tessera) is `NotAvailable` with
/// that why and nothing is looked up. Otherwise each name the entry `needs` is checked in order, and the first not
/// found is `NotOnPath` naming it: the entry's own program counts as found with no lookup when `program_for` gives a
/// local one in `folder` (ESLint's `node_modules/.bin/eslint`); every other name (node) is looked up with `on_path`.
pub(crate) fn unavailable(engine: Engine, folder: Option<&Path>, on_path: impl Fn(&str) -> bool) -> Option<Stop> {
    let entry = entry_of(engine);
    if let Some(why) = entry.never {
        return Some(Stop::NotAvailable { why: why.to_string() });
    }
    let local = program_for(entry, folder) != entry.program;
    let found = |name: &&str| (local && *name == entry.program) || on_path(name);
    let missing = entry.needs.iter().find(|name| !found(name))?;
    Some(Stop::NotOnPath { program: missing.to_string() })
}
