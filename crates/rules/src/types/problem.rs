//! `Problem`: one thing wrong in a rule file.

/// A problem in a rule file at a 1-based line and column (the column counts chars); line 0, col 0 means there is no
/// position, as when the file cannot be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    pub line: usize,
    pub col: usize,
    /// What is wrong, naming the key or value.
    pub what: String,
    /// What to write instead.
    pub fix: String,
}
