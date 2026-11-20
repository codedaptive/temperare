//! `run_shellcheck`: shellcheck's flow for one rule, against temperare's compiled-in list of shellcheck codes.

use super::{run_shellcheck_with, shell_files};
use crate::entry::Entry;
use crate::host::Host;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// `run_shellcheck_with` on `shellcheck_codes.txt`, the list compiled in, listing the scripts with `shell_files`; a
/// `RunFn`, the flow of `SHELLCHECK`.
pub(crate) fn run_shellcheck(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    run_shellcheck_with(include_str!("shellcheck_codes.txt"), &shell_files, entry, rule, folder, host)
}
