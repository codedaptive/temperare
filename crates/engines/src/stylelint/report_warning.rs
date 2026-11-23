//! `ReportWarning`: one problem stylelint's JSON report gives a file.

use serde::Deserialize;

/// One entry of a file's `warnings`, `invalidOptionWarnings` or `parseErrors`. A `warnings` entry carries all four
/// fields; an `invalidOptionWarnings` entry carries `text` alone. Its end position and severity are not read.
#[derive(Debug, PartialEq, Deserialize)]
pub(crate) struct ReportWarning {
    /// The rule's name; `CssSyntaxError` for a file stylelint could not parse.
    pub(crate) rule: Option<String>,
    pub(crate) line: Option<u32>,
    pub(crate) column: Option<u32>,
    pub(crate) text: String,
}
