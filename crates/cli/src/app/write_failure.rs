//! `write_failure`: a usage error as a subcommand's exit 2, on stderr and, under `--json`, as an object on stdout.

use super::write_usage_error::write_usage_error;
use crate::UsageError;
use serde_json::json;
use std::io::Write;

/// Writes `e` on `err` as cctao's usage error and, when `json`, `{"error": <what>, "exit": 2, "next": <fix>}` as one
/// line on `out`; returns 2. Write errors are ignored: a closed pipe is not cctao's failure.
pub(crate) fn write_failure(e: &UsageError, json: bool, out: &mut dyn Write, err: &mut dyn Write) -> u8 {
    write_usage_error(err, e);
    if json {
        let _ = writeln!(out, "{}", json!({"error": e.what, "exit": 2, "next": e.fix}));
    }
    2
}
