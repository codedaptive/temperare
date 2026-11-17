//! `UsageError`: a command line tm does not take, and what to type instead.

/// What was wrong with the command line (`what`) and one sentence telling the user what to type instead (`fix`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageError {
    pub what: String,
    pub fix: String,
}
