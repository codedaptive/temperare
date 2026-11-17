//! `problem_at`: a Problem at a byte offset of the rule-file text.

use crate::Problem;

/// The Problem `what` / `fix` at byte `offset` of `text`, with its 1-based line and column (the column counts chars).
pub(crate) fn problem_at(text: &str, offset: usize, what: String, fix: &str) -> Problem {
    let before = text.get(..offset).unwrap_or(text);
    let line_start = before.rfind('\n').map_or(0, |i| i + 1);
    Problem {
        line: before.matches('\n').count() + 1,
        col: before[line_start..].chars().count() + 1,
        what,
        fix: fix.to_string(),
    }
}
