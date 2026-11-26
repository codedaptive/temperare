//! `availability_at`: whether tm finds an engine for a rule file in a folder, asked of the folder's canonical path.
//! It uses the real disk and PATH, and a ZUT uses only pure code (rules/global.md rule 11), so its tests are not
//! ZUTs: integration/availability_at_tests.rs on the real disk, and tests/world/local_eslint_real.rs through the
//! binary.

use engines::Availability;
use rules::Engine;
use std::path::Path;

/// Whether `engine` can be run for a rule file in `folder`: `folder` is canonicalized and handed to
/// `engines::availability_in` (ESLint from the folder's node_modules/.bin, else PATH); nothing is run. A folder that
/// cannot be canonicalized gives `NotAvailable`, its why naming the folder and the io error; it never falls back to a
/// PATH-only answer.
pub(crate) fn availability_at(engine: Engine, folder: &Path) -> Availability {
    match std::fs::canonicalize(folder) {
        Ok(canonical) => engines::availability_in(engine, &canonical),
        Err(e) => Availability::NotAvailable {
            why: format!("cannot resolve the rule file's folder {}: {e}", folder.display()),
        },
    }
}
