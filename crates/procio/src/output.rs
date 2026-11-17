//! `Output`: what a program that ran to its end left behind.

/// The exit code and everything the program wrote. Any exit code is an answer; procio gives none a meaning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output {
    /// The exit code; `None` when a signal ended the program.
    pub code: Option<i32>,
    /// Everything written to stdout, in full.
    pub stdout: Vec<u8>,
    /// Everything written to stderr, in full.
    pub stderr: Vec<u8>,
}
