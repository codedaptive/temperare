//! `parse_failed`: an engine output that was not an answer, as a `Stop`.

use crate::types::{ParseError, Stop};
use procio::Output;

/// `Failed` with the engine's exit code. `stderr` is what the engine wrote to stderr (lossy UTF-8) when it wrote
/// anything, since that is the engine's own account of the failure; else the parser's `why`.
pub(crate) fn parse_failed(out: &Output, err: ParseError) -> Stop {
    let stderr = if out.stderr.is_empty() { err.why } else { String::from_utf8_lossy(&out.stderr).into_owned() };
    Stop::Failed { code: out.code, stderr }
}
