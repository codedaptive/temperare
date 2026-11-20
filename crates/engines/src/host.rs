//! `Host`: what running the rules reaches outside the process for.

use procio::{Cmd, Output, ProcError};

/// The two things a rule run asks of the machine: run a program, and look a program up on PATH. Production passes
/// `procio::run` and `procio::which`; a unit test passes fakes, so no engine runs in it.
pub(crate) struct Host<'a> {
    pub(crate) run: &'a dyn Fn(&Cmd) -> Result<Output, ProcError>,
    pub(crate) on_path: &'a dyn Fn(&str) -> bool,
}
