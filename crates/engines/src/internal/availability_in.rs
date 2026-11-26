//! `availability_in`: whether an engine can be run for a rule file's folder. It uses the real PATH and disk, so it has
//! no unit-test sibling. The zone integration test in integration/eslint_local_bin_tests.rs calls it with the real
//! PATH, and tests its body, one call to `availability_from`, with PATH given.

use crate::availability_from::availability_from;
use crate::types::Availability;
use rules::Engine;
use std::path::Path;

/// Whether `engine` can be run for a rule file in `folder`, its canonical folder, with the same program the run uses
/// (`program_for`): ESLint's is `<folder>/node_modules/.bin/eslint` when that is a file, else PATH's; every other
/// name, `node` included, is looked up on PATH. Nothing is run.
pub fn availability_in(engine: Engine, folder: &Path) -> Availability {
    availability_from(engine, Some(folder), |name| procio::which(name).is_some())
}
