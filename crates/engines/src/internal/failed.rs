//! `failed`: a failure cctao found itself, not one an engine's exit reported, as a `Stop`.

use crate::types::Stop;

/// `Failed` with no exit code, since no engine exit settled it; `stderr` is cctao's own account of the failure.
pub(crate) fn failed(stderr: String) -> Stop {
    Stop::Failed { code: None, stderr }
}
