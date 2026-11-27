//! `SarifFailure`: why a SARIF log gave no hits for a rule, kept apart so a reader can word each case its own way.

use crate::types::ParseError;

/// How reading a SARIF log failed.
#[derive(Debug, PartialEq)]
pub(crate) enum SarifFailure {
    /// stdout is not a SARIF log (not JSON, or no `runs` array); its `why` quotes the start of what the engine printed.
    NotALog(ParseError),
    /// A result of a checked rule names a path not under the rule folder; its `why` quotes that URI and the folder.
    Unplaced(ParseError),
}
