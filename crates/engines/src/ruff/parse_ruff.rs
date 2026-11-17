//! `parse_ruff`: ruff's JSON output, read into the hits of one rule.

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use serde::Deserialize;
use std::path::Path;

/// One diagnostic as `ruff check --output-format json` prints it; other fields (fix, url, end_location) are not read.
#[derive(Deserialize)]
struct Diagnostic {
    /// Null for a diagnostic that is no rule's, such as a syntax error in some ruff versions; it matches no check.
    code: Option<String>,
    filename: String,
    location: Location,
    message: String,
}

/// Where a diagnostic starts: ruff's row and column, both 1-based.
#[derive(Deserialize)]
struct Location {
    row: u32,
    column: u32,
}

/// The hits of rule `check` in `out`, in output order. stdout is one JSON array of diagnostics; any exit code with
/// such stdout is an answer (ruff exits 1 when it finds any). `cwd` is where ruff ran and `folder` the rule file's
/// folder, both canonical; a diagnostic in a file outside `folder` is not a hit. stdout that is not that array is a
/// `ParseError` quoting the first 200 bytes of stderr, or of stdout when stderr is empty.
pub fn parse_ruff(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    let diagnostics: Vec<Diagnostic> = serde_json::from_slice(&out.stdout).map_err(|_| not_findings(out))?;
    Ok(diagnostics
        .into_iter()
        .filter(|diagnostic| diagnostic.code.as_deref() == Some(check))
        .filter_map(|diagnostic| {
            Some(Hit {
                path: rel_to_folder(cwd, folder, &diagnostic.filename)?,
                line: diagnostic.location.row,
                col: Some(diagnostic.location.column),
                what: diagnostic.message,
            })
        })
        .collect())
}

/// The error for output that is not ruff's JSON: it names ruff and quotes what it printed.
fn not_findings(out: &Output) -> ParseError {
    ParseError { why: format!("ruff printed no JSON findings: {}", printed_start(out)) }
}
