//! `ran`: one engine run through the host, its errors read as a `Stop`.

use crate::host::Host;
use crate::types::Stop;
use procio::{Cmd, Output, ProcError};

/// The output of `cmd` run by `host`, whatever its exit code. A program not found is `NotAvailable` with the why
/// form `availability` uses, `<program> not found on PATH`; a program that could not be run or timed out is `Failed` with no exit code and
/// procio's one-line error, which names the program.
pub(crate) fn ran(cmd: &Cmd, host: &Host) -> Result<Output, Stop> {
    (host.run)(cmd).map_err(|err| match err {
        ProcError::NotFound { program } => {
            Stop::NotAvailable { why: format!("{} not found on PATH", program.to_string_lossy()) }
        }
        other => Stop::Failed { code: None, stderr: other.to_string() },
    })
}
