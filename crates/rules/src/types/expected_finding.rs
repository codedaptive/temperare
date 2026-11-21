//! `ExpectedFinding`: the finding a baseline rule's failing case must give.

/// The one finding a failing case must give when its rule is run on the case's files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedFinding {
    /// The id of the rule the case belongs to.
    pub rule_id: String,
    /// The finding's file, relative to the folder the case is run in.
    pub path: String,
    /// The finding's 1-based line.
    pub line: usize,
}
