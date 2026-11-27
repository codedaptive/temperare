//! `FileReport`: one file's result in stylelint's JSON report.

use super::ReportWarning;
use serde::Deserialize;

/// One linted file as `stylelint --formatter json` prints it; `deprecations` and `errored` are not read. stylelint
/// prints `ignored` and `autofixed` only when they are true, so each is false when absent.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct FileReport {
    /// The file's absolute path, as stylelint resolved it.
    pub(crate) source: String,
    /// True when the config's `ignoreFiles` covers the file: stylelint did not lint it.
    #[serde(default)]
    pub(crate) ignored: bool,
    /// True when the config's `fix` made stylelint rewrite the file.
    #[serde(default)]
    pub(crate) autofixed: bool,
    #[serde(default)]
    pub(crate) warnings: Vec<ReportWarning>,
    #[serde(default)]
    pub(crate) invalid_option_warnings: Vec<ReportWarning>,
    #[serde(default)]
    pub(crate) parse_errors: Vec<ReportWarning>,
}
