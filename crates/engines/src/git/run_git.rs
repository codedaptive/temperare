//! `run_git`: the git entry's flow for one rule, on the real change.

use super::run_git_with;
use crate::change_of::change_of;
use crate::entry::Entry;
use crate::host::Host;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// `run_git_with` reading the change with `change_of` (git, through ccvcslib); a `RunFn`, the flow of `GIT`.
pub(crate) fn run_git(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    run_git_with(&change_of, entry, rule, folder, host)
}
