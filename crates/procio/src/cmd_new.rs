//! `Cmd::new`: a command with only its program set.

use std::ffi::OsString;

use crate::Cmd;

impl Cmd {
    /// A command for `program` with no arguments, no cwd, no environment changes, empty stdin and no timeout.
    /// Callers set the other fields with struct update syntax: `Cmd { args, ..Cmd::new("git") }`.
    pub fn new(program: impl Into<OsString>) -> Cmd {
        Cmd {
            program: program.into(),
            args: Vec::new(),
            cwd: None,
            env: Vec::new(),
            env_remove: Vec::new(),
            stdin: Vec::new(),
            timeout: None,
        }
    }
}
