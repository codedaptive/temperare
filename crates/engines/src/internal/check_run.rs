//! `check_run`: a rule's check run, the run whose output becomes findings, counted as it is run.

use crate::host::Host;
use crate::ran::ran;
use crate::types::Stop;
use procio::{Cmd, Output};

/// `ran(cmd, host)`, and when it gives an `Output`, whatever its exit code, `host.raw_chars` is set to the characters
/// of its stdout plus its stderr, each read as lossy UTF-8 (each invalid sequence lossy decoding replaces counts as one
/// character). A run that gives no `Output` leaves `host.raw_chars` as it was and returns `ran`'s `Stop`.
pub(crate) fn check_run(cmd: &Cmd, host: &Host) -> Result<Output, Stop> {
    let out = ran(cmd, host)?;
    host.raw_chars.set(Some(
        String::from_utf8_lossy(&out.stdout).chars().count() + String::from_utf8_lossy(&out.stderr).chars().count(),
    ));
    Ok(out)
}
