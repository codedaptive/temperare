//! `run_vulture`: vulture's flow for one rule, the entry's `RunFn`, leaving out what git ignores. It is
//! `run_vulture_with` over the real `ignored_python`, which runs git, so it has no unit-test sibling; the zone
//! integration test tests/vulture.rs covers it.

use super::{ignored_python, run_vulture_with};
use crate::entry::Entry;
use crate::host::Host;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// `run_vulture_with` given `ignored_python`, so what git ignores under `folder` is left out of the scan.
pub(crate) fn run_vulture(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    run_vulture_with(&ignored_python, entry, rule, folder, host)
}
