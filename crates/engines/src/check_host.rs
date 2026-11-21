//! `check_host`: the host one check runs its rules through in production.

use crate::host::Host;
use std::cell::Cell;

/// The production host: programs run by `procio::run`, looked up on PATH by `procio::which`, check runs counted in
/// `raw_chars`, and `base` the commit the check compares against, `None` without one.
pub(crate) fn check_host<'a>(base: Option<&'a str>, raw_chars: &'a Cell<Option<usize>>) -> Host<'a> {
    Host { run: &procio::run, on_path: &on_path, raw_chars, base }
}

/// Whether `program` is found on PATH.
fn on_path(program: &str) -> bool {
    procio::which(program).is_some()
}
