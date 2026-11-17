//! `spawn_error`: the `ProcError` for a program that failed to start.

use std::ffi::OsStr;
use std::io;

use crate::ProcError;

/// `NotFound` when the OS said "no such file" and the program was not found by `which`; `Spawn` otherwise.
///
/// `found` is needed because a missing cwd also fails with "no such file": a program that exists must then stay
/// `Spawn`, or callers would report a present engine as "not available".
pub(crate) fn spawn_error(program: &OsStr, io: io::Error, found: bool) -> ProcError {
    let program = program.to_os_string();
    if io.kind() == io::ErrorKind::NotFound && !found {
        return ProcError::NotFound { program };
    }
    ProcError::Spawn { program, io }
}
