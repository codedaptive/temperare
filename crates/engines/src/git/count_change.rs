//! `count_change`: a change's added lines, counted as test or code.

use super::{Counts, TestRatio};
use rules::{RelPath, Rule};
use std::ops::Range;

/// Counts the added lines of the `files` the rule applies to (paths relative to the rule file's folder, each with
/// its added lines as 1-based half-open ranges): a file `test_paths` matches counts as test, any other as code.
/// `top_test` is the test file with the most added lines, ties to the smallest path in `RelPath` order, with its
/// smallest range start.
pub(crate) fn count_change(rule: &Rule, ratio: &TestRatio, files: &[(RelPath, Vec<Range<u32>>)]) -> Counts {
    let mut counts = Counts { test: 0, code: 0, top_test: None };
    let mut top_lines = 0;
    for (path, ranges) in files.iter().filter(|(path, _)| rule.applies(path)) {
        let lines: u32 = ranges.iter().map(|r| r.end.saturating_sub(r.start)).sum();
        if !ratio.test_paths.matches(path) {
            counts.code += lines;
            continue;
        }
        counts.test += lines;
        let first = ranges.iter().map(|r| r.start).min();
        let beats = match &counts.top_test {
            None => lines > 0,
            Some((top, _)) => lines > top_lines || (lines == top_lines && path < top),
        };
        if let (true, Some(first)) = (beats, first) {
            counts.top_test = Some((path.clone(), first));
            top_lines = lines;
        }
    }
    counts
}
