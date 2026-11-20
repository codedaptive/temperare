//! `Host`: what running the rules reaches outside the process for, and where a rule's check run is counted.

use procio::{Cmd, Output, ProcError};
use std::cell::Cell;

/// Runs a program, looks a program up on PATH, and records the character count of a rule's check run in
/// `raw_chars` (`check_run` sets it). Production passes `procio::run`, `procio::which` and a fresh cell; a ZUT
/// passes fakes, so no engine runs in it.
pub(crate) struct Host<'a> {
    pub(crate) run: &'a dyn Fn(&Cmd) -> Result<Output, ProcError>,
    pub(crate) on_path: &'a dyn Fn(&str) -> bool,
    pub(crate) raw_chars: &'a Cell<Option<usize>>,
}
