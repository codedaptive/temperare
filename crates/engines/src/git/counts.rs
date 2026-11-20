//! `Counts`: the lines a change adds, split into test and code.

use rules::RelPath;

/// The added lines of test and of code in a change, and the test file with the most added lines paired with its
/// first added line (None when no test file adds a line).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Counts {
    pub test: u32,
    pub code: u32,
    pub top_test: Option<(RelPath, u32)>,
}
