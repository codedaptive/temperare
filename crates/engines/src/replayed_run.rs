//! `replayed_run`: a command run once for the rules that share it, its result replayed to the rest.

use crate::proc_error_copy::proc_error_copy;
use procio::{Cmd, Output, ProcError};
use std::cell::RefCell;

/// The result of `cmd` under `key`: when `seen` already holds `key`, a copy of the result stored with it, and nothing
/// runs; else `cmd` run through `run` once, its result stored under `key` and a copy returned. The copy is the
/// `Output` cloned, or the `ProcError` copied by `proc_error_copy`, so every rule replaying one failed run reads the
/// same failure. `key` and `cmd` differ only where a rule's own run carries something no other rule's has (an
/// ast-grep rule's filled YAML copy, whose temporary path is different for each rule).
pub(crate) fn replayed_run(
    seen: &RefCell<Vec<(Cmd, Result<Output, ProcError>)>>,
    key: &Cmd,
    cmd: &Cmd,
    run: &dyn Fn(&Cmd) -> Result<Output, ProcError>,
) -> Result<Output, ProcError> {
    if let Some((_, got)) = seen.borrow().iter().find(|(stored, _)| stored == key) {
        return copied(got);
    }
    let got = run(cmd);
    let copy = copied(&got);
    seen.borrow_mut().push((key.clone(), got));
    copy
}

fn copied(got: &Result<Output, ProcError>) -> Result<Output, ProcError> {
    got.as_ref().map(Output::clone).map_err(proc_error_copy)
}
