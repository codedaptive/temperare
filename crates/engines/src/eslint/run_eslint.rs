//! `run_eslint`: ESLint's flow for one rule, the entry's `RunFn`, keeping the files git lists. It is
//! `run_eslint_with` over the real `files_in_folder`, which runs git, so it has no unit-test sibling; the zone
//! integration test tests/eslint.rs covers it.

use super::run_eslint_with;
use crate::entry::Entry;
use crate::files_in_folder::files_in_folder;
use crate::host::Host;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// `run_eslint_with` given `files_in_folder`, so a run that lints no regular file git lists for `folder` and the
/// rule's paths cover is no file, and a hit in a file git ignores is dropped.
pub(crate) fn run_eslint(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    run_eslint_with(&files_in_folder, entry, rule, folder, host)
}
