//! `stylelint_report`: the JSON report a stylelint run printed, read per file.

use super::FileReport;
use procio::Output;

/// The file results in `out`: the last stderr line (lossy UTF-8, trimmed) that starts with `[`, read as stylelint's
/// JSON report. stylelint writes its JSON report to stderr, as one line, after any Node warning lines; stdout is not
/// read. None when no stderr line starts with `[` or that line is not the report.
pub(crate) fn stylelint_report(out: &Output) -> Option<Vec<FileReport>> {
    let stderr = String::from_utf8_lossy(&out.stderr);
    let line = stderr.lines().rev().map(str::trim).find(|line| line.starts_with('['))?;
    serde_json::from_str(line).ok()
}
