//! `parse_actionlint`: actionlint's JSON output, read into the hits of one rule.

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use serde::Deserialize;
use std::path::Path;

/// What the entry's template prints on stdout; `kinds` is read by `unknown_kind`, not here.
#[derive(Deserialize)]
struct Report {
    errors: Vec<Error>,
}

/// One error; `snippet` and `end_column` are not read.
#[derive(Deserialize)]
struct Error {
    message: String,
    /// The workflow's path as it was passed, relative to the run's working folder.
    filepath: String,
    line: u32,
    column: u32,
    kind: String,
}

/// The hits in `out`: the errors whose `kind` is `check`, in actionlint's order. A hit's path is the error's
/// `filepath` made relative to `folder` from `cwd` (both canonical); an error outside `folder` gives no hit. Its line
/// and column are the error's, None when 0; its text the message. actionlint exits 1 when it finds anything and 3 when
/// it fails, so the exit code is not read: the JSON decides. Stdout that is not that JSON is a `ParseError` naming
/// actionlint and quoting `printed_start`.
pub fn parse_actionlint(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    let report: Report = serde_json::from_slice(&out.stdout)
        .map_err(|_| ParseError { why: format!("actionlint printed no JSON report: {}", printed_start(out)) })?;
    Ok(report
        .errors
        .into_iter()
        .filter(|error| error.kind == check)
        .filter_map(|error| {
            Some(Hit {
                path: rel_to_folder(cwd, folder, &error.filepath)?,
                line: Some(error.line).filter(|&line| line > 0),
                col: Some(error.column).filter(|&col| col > 0),
                what: error.message,
            })
        })
        .collect())
}
