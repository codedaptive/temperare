//! `ProcError`: the ways `run` can fail to give an answer. A non-zero exit is not one of them.

use std::ffi::OsString;
use std::time::Duration;

/// Why a program gave no `Output`. Every variant names the program as the caller gave it.
#[derive(Debug)]
pub enum ProcError {
    /// The program was not found: a name in no absolute entry of the `Cmd`'s effective PATH (on Windows, with none
    /// of the PATHEXT names either), or a path that does not exist. Kept apart from `Spawn` because engines read it as
    /// "not available", which is never "clean".
    NotFound { program: OsString },
    /// Any other OS failure: to start the program, to write its stdin, to read its stdout or stderr, or to wait on it.
    Spawn { program: OsString, io: std::io::Error },
    /// The caller's timeout passed; the program was killed and reaped, and on unix its process group was sent a
    /// kill whose result is not checked. `after` is the caller's timeout.
    TimedOut { program: OsString, after: Duration },
}
