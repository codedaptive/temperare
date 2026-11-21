//! `Host`: what running the rules reaches outside the process for, where a rule's check run is counted, and the
//! base commit the check compares against.

use procio::{Cmd, Output, ProcError};
use std::cell::Cell;

/// Runs a program, looks a program up on PATH, records the character count of a rule's check run in `raw_chars`
/// (`check_run` sets it), and carries `base`. Production passes `procio::run`, `procio::which` and a fresh cell; a ZUT
/// passes fakes, so no engine runs in it.
pub(crate) struct Host<'a> {
    pub(crate) run: &'a dyn Fn(&Cmd) -> Result<Output, ProcError>,
    pub(crate) on_path: &'a dyn Fn(&str) -> bool,
    pub(crate) raw_chars: &'a Cell<Option<usize>>,
    /// The commit the check compares against (cli's `--base`), as cli gives it; `None` on a check without one. An
    /// entry that needs it reads it from the host it is given and, finding `None`, stops with
    /// `rules::ONLY_WITH_BASE_WHY`; the git entry and ast-grep's count mode (`count_ast_grep`) do, and no other entry
    /// reads it.
    pub(crate) base: Option<&'a str>,
}
