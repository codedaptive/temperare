//! `run_rubocop`: rubocop's flow for one rule, the entry's `RunFn`, reading the folder's files from disk. It is
//! `run_rubocop_with` over `std::fs::read_to_string`, so it has no unit-test sibling; the zone integration test
//! tests/rubocop.rs covers it.

use super::run_rubocop_with;
use crate::entry::Entry;
use crate::host::Host;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// `run_rubocop_with` reading each file it asks for from disk; a file that cannot be read as text reads as absent.
pub(crate) fn run_rubocop(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    run_rubocop_with(&|path| std::fs::read_to_string(path).ok(), entry, rule, folder, host)
}
