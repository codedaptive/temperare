//! `ParseError`: an engine's output that was not an answer.

/// The engine's output could not be read as findings (not its format, or a build that failed), so the run says
/// nothing about the code; `why` names the engine and quotes the start of what it printed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub why: String,
}
