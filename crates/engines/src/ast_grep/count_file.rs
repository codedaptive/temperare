//! `count_file`: one file's setup and test lines, from its ast-grep matches.

use std::cmp::Reverse;
use std::ops::RangeInclusive;

use super::{CountMatch, FileCount};

/// Counts one file from its matches, in output order. The root node is the match that starts first and, among
/// those, ends last (the first in output order on a tie); every other match is a test function. The file's lines are
/// the root's own text, numbered from its start line, so no source file is read. A line is blank when it is empty
/// after `trim()`; every other line counts as test when a test function covers it and as setup otherwise. None when
/// there is no test function: such a file holds no tests and is never reported.
pub(crate) fn count_file(matches: &[&CountMatch]) -> Option<FileCount> {
    let (root_at, root) = matches
        .iter()
        .enumerate()
        .min_by_key(|(at, found)| (found.start_byte, Reverse(found.end_byte), *at))?;
    let tests: Vec<RangeInclusive<u32>> =
        matches.iter().enumerate().filter(|(at, _)| *at != root_at).map(|(_, found)| covered(found)).collect();
    if tests.is_empty() {
        return None;
    }
    let mut count = FileCount { setup: 0, tests: 0 };
    for (number, line) in (root.start_line..).zip(root.text.lines()) {
        if line.trim().is_empty() {
            continue;
        }
        if tests.iter().any(|lines| lines.contains(&number)) {
            count.tests += 1;
        } else {
            count.setup += 1;
        }
    }
    Some(count)
}

/// The lines a test function covers. A match that ends at column 0 of a later line ends with that line's newline,
/// so the line itself holds none of the match.
fn covered(found: &CountMatch) -> RangeInclusive<u32> {
    let ends_before_line = found.end_column == 0 && found.end_line > found.start_line;
    found.start_line..=if ends_before_line { found.end_line - 1 } else { found.end_line }
}
