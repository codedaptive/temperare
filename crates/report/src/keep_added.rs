//! `keep_added`: the findings on lines added since the base commit, or under an untracked prefix entry.

use rules::Finding;
use std::collections::BTreeMap;
use std::ops::Range;
use std::path::{Component, Path, PathBuf};

/// Keeps each finding whose path, re-rooted from the rule file's `folder` to the repository top, is a key of `added`
/// with a range holding its line, or lies under an entry of `under` (an untracked nested repository) at any line, a
/// finding with no line included; drops every other finding (a finding with no line is on no added line) and keeps the
/// order. `added` has `procio::added_lines`' shape: keys relative to the repository top, 1-based half-open line
/// ranges. `under` entries are relative to the repository top and match whole components (`nested/` covers
/// `nested/deep/d.rs`, never `nestedx/c.rs`). `folder` is relative to the repository top; "" and "." both mean the
/// top.
pub fn keep_added(
    findings: Vec<Finding>,
    added: &BTreeMap<PathBuf, Vec<Range<u32>>>,
    under: &[PathBuf],
    folder: &Path,
) -> Vec<Finding> {
    findings
        .into_iter()
        .filter(|finding| {
            let path = from_top(folder, finding);
            under.iter().any(|prefix| path.starts_with(prefix))
                || finding.line().is_some_and(|line| {
                    added.get(&path).is_some_and(|ranges| ranges.iter().any(|range| range.contains(&line)))
                })
        })
        .collect()
}

/// `folder` joined with the finding's path, every "." component dropped, so the result compares equal to a key of
/// `added` (`Path::new(".").join("src/a.rs")` keeps its "./" and would not).
fn from_top(folder: &Path, finding: &Finding) -> PathBuf {
    folder
        .components()
        .chain(Path::new(finding.path.as_str()).components())
        .filter(|component| *component != Component::CurDir)
        .collect()
}
