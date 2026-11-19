//! `stderr_tail`: the end of a failed engine's stderr, as the report carries it.

/// How many lines of a failed engine's stderr the report keeps: the last 20.
pub(crate) const STDERR_TAIL_LINES: usize = 20;

/// The last `STDERR_TAIL_LINES` lines of `stderr` (split by `str::lines`), joined with '\n' and no trailing newline;
/// all of them when there are fewer.
pub(crate) fn stderr_tail(stderr: &str) -> String {
    let lines: Vec<&str> = stderr.lines().collect();
    lines[lines.len().saturating_sub(STDERR_TAIL_LINES)..].join("\n")
}
