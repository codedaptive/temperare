//! `run_shellcheck`: shellcheck's flow for one rule, against temperare's compiled-in list of shellcheck codes.

use super::run_shellcheck_with;
use crate::entry::Entry;
use crate::host::Host;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// `run_shellcheck_with` on `shellcheck_codes.txt`, the list compiled in; a `RunFn`.
// No entry runs it yet, because rules::Engine has no ShellCheck engine; only its tests call it.
#[allow(dead_code)]
pub(crate) fn run_shellcheck(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    run_shellcheck_with(include_str!("shellcheck_codes.txt"), entry, rule, folder, host)
}
