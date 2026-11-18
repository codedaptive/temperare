//! `parse_ruff`: ruff's JSON output, read into the hits of one rule.

use super::ruff_selects;
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

/// The hits of rule `check` in `out`, in output order: the diagnostics whose code `check` selects as a ruff
/// `--select` list (`ruff_selects`: a code, a code prefix or ALL, comma-separated). stdout is one JSON array of
/// diagnostics; any exit code with such stdout is an answer (ruff exits 1 when it finds any). `cwd` is where ruff ran
/// and `folder` the rule file's folder, both canonical; a diagnostic in a file outside `folder` is not a hit. stdout
/// that is not that array is a `ParseError` quoting the first 200 bytes of stderr, or of stdout when stderr is empty.
/// A stderr line saying a code "has been remapped to" another is a `ParseError` too when `check` selects the old code
/// (the line's first backticked code): ruff then reports under the new code, which `check` may not select. A remap of
/// a code `check` does not select (one the project's own config names) is not an error, and a line with no
/// backticked code is ignored.
pub fn parse_ruff(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    if selects_a_remapped_code(out, check) {
        return Err(ParseError {
            why: format!("ruff check `{check}` names a remapped code, so its findings carry another code: {}", printed_start(out)),
        });
    }
    let diagnostics: Vec<Diagnostic> = serde_json::from_slice(&out.stdout).map_err(|_| not_findings(out))?;
    Ok(diagnostics
        .into_iter()
        .filter(|diagnostic| diagnostic.code.as_deref().is_some_and(|code| ruff_selects(check, code)))
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

/// Whether some stderr line of `out` says a code "has been remapped to" another, as in ruff 0.17.4's
/// "warning: `TCH001` has been remapped to `TC001`.", and `check` selects that line's first backticked code.
fn selects_a_remapped_code(out: &Output, check: &str) -> bool {
    String::from_utf8_lossy(&out.stderr)
        .lines()
        .filter(|line| line.contains("has been remapped to"))
        .filter_map(|line| line.split_once('`')?.1.split_once('`').map(|(old_code, _)| old_code))
        .any(|old_code| ruff_selects(check, old_code))
}

/// The error for output that is not ruff's JSON: it names ruff and quotes what it printed.
fn not_findings(out: &Output) -> ParseError {
    ParseError { why: format!("ruff printed no JSON findings: {}", printed_start(out)) }
}
