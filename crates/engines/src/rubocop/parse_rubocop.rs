//! `parse_rubocop`: rubocop's JSON report, read into the hits of one rule.

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use serde::Deserialize;
use std::path::Path;

/// The report `rubocop --format json` prints as one object on stdout; its metadata and summary are not read.
#[derive(Deserialize)]
struct Report {
    files: Vec<File>,
}

/// One inspected file, its path relative to where rubocop ran; a clean file has no offenses.
#[derive(Deserialize)]
struct File {
    path: String,
    offenses: Vec<Offense>,
}

/// One offense; its correction and the rest of its location are not read.
#[derive(Deserialize)]
struct Offense {
    severity: String,
    message: String,
    cop_name: String,
    location: Location,
}

/// Where an offense starts: rubocop's line and column, both 1-based.
#[derive(Deserialize)]
struct Location {
    start_line: u32,
    start_column: u32,
}

/// The hits of rule `check` in `out`, in output order: each offense whose `cop_name` is one of `check`'s
/// comma-separated cops, at its file's path made relative to `folder` (an offense in a file outside `folder` is not
/// a hit), its start line and column, and rubocop's message, which carries no cop name. `cwd` is where rubocop ran
/// and `folder` the rule file's folder, both canonical.
///
/// Exit 0 (no offenses) and 1 (offenses) are answers; any other exit is a `ParseError`, even with a report on stdout:
/// rubocop exits 2 for an unknown cop, a config it cannot load and a crash, after which its report is not the whole
/// answer. stdout that is not the report is a `ParseError` too; both quote `printed_start`. An offense of severity
/// `fatal` whose cop is not one of `check`'s (`Lint/Syntax` for a file rubocop could not parse, which it reports
/// whatever `--only` says) is a `ParseError` naming the file and the message, so a rule never reads clean over a file
/// rubocop could not read.
pub fn parse_rubocop(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    if !matches!(out.code, Some(0 | 1)) {
        let exit = match out.code {
            Some(code) => format!("exited {code}"),
            None => "was stopped by a signal".to_string(),
        };
        return Err(ParseError { why: format!("rubocop {exit}, not 0 or 1: {}", printed_start(out)) });
    }
    let report: Report = serde_json::from_slice(&out.stdout)
        .map_err(|_| ParseError { why: format!("rubocop printed no JSON report: {}", printed_start(out)) })?;
    let cops: Vec<&str> = check.split(',').collect();
    let mut hits = Vec::new();
    for file in report.files {
        for offense in file.offenses {
            if !cops.contains(&offense.cop_name.as_str()) {
                if offense.severity == "fatal" {
                    return Err(ParseError { why: format!("rubocop could not read {}: {}", file.path, offense.message) });
                }
                continue;
            }
            if let Some(path) = rel_to_folder(cwd, folder, &file.path) {
                let Location { start_line, start_column } = offense.location;
                hits.push(Hit { path, line: Some(start_line), col: Some(start_column), what: offense.message });
            }
        }
    }
    Ok(hits)
}
