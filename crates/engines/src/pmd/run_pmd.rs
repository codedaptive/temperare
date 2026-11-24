//! `run_pmd`: PMD's flow for one rule, the entry's `RunFn`. It is `run_pmd_with` over the real `java_files`, which
//! runs git, so it has no unit-test sibling; the zone integration test tests/pmd.rs covers it.

use super::{java_files, run_pmd_with};
use crate::entry::Entry;
use crate::host::Host;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// `run_pmd_with` given `java_files`, so PMD checks the `.java` files git lists under `folder`.
pub(crate) fn run_pmd(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    run_pmd_with(&java_files, entry, rule, folder, host)
}
