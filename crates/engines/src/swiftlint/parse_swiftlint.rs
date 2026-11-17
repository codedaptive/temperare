//! `parse_swiftlint`: swiftlint's JSON reporter output, read into the hits of one rule.

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use serde::Deserialize;
use std::path::Path;

/// One violation as swiftlint's JSON reporter prints it; other fields (severity, type) are not read.
#[derive(Deserialize)]
struct Violation {
    file: String,
    line: u32,
    /// Null for a violation of a whole line or file (trailing_newline, for one).
    character: Option<u32>,
    reason: String,
    rule_id: String,
}

/// The hits of rule `check` in `out`, in output order. stdout is one JSON array of violations; any exit code with
/// such stdout is an answer (swiftlint exits 2 when a violation is an error). `cwd` is where swiftlint ran and
/// `folder` the rule file's folder, both canonical; a violation in a file outside `folder` is not a hit. stdout that
/// is not that array is a `ParseError` quoting the first 200 bytes of stderr, or of stdout when stderr is empty.
pub fn parse_swiftlint(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    let violations: Vec<Violation> = serde_json::from_slice(&out.stdout).map_err(|_| not_findings(out))?;
    Ok(violations
        .into_iter()
        .filter(|violation| violation.rule_id == check)
        .filter_map(|violation| {
            Some(Hit {
                path: rel_to_folder(cwd, folder, &violation.file)?,
                line: violation.line,
                col: violation.character,
                what: violation.reason,
            })
        })
        .collect())
}

/// The error for output that is not swiftlint's JSON: it names swiftlint and quotes what it printed.
fn not_findings(out: &Output) -> ParseError {
    ParseError { why: format!("swiftlint printed no JSON findings: {}", printed_start(out)) }
}
