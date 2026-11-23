//! `parse_periphery`: Periphery's JSON results, read into the hits of one rule.

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use serde::Deserialize;
use std::path::Path;

/// The fields of one result tm reads. Only these: a `redundantConformance` result carries no `modules`, and the
/// other fields (`ids`, `modifiers`, `attributes`, `accessibility`) say nothing tm reports.
#[derive(Deserialize)]
struct Scanned {
    name: String,
    kind: String,
    hints: Vec<String>,
    location: String,
}

/// The hits of rule `check` in `periphery scan --format json`'s output `out`, a JSON array of results, any exit code:
/// each result whose `hints` hold `check` (every result for `all`), in Periphery's order. Its place is its
/// `location`, `<path>:<line>:<col>`, split from the right since the path may hold `:`; the path, absolute or relative
/// to `cwd`, is made relative to `folder` (`rel_to_folder`), and a result outside `folder` is skipped. Its `what`
/// reads `<kind> <name>: <hints joined with ", ">`. stdout that is not the array, or a location that does not read so,
/// is a `ParseError`.
pub fn parse_periphery(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    let results: Vec<Scanned> = serde_json::from_slice(&out.stdout)
        .map_err(|_| ParseError { why: format!("periphery printed no JSON result list: {}", printed_start(out)) })?;
    let mut hits = Vec::new();
    for result in results.into_iter().filter(|result| check == "all" || result.hints.iter().any(|hint| hint == check)) {
        let (path, line, col) = place(&result.location).ok_or_else(|| ParseError {
            why: format!("periphery printed a location tm cannot read: `{}`", result.location),
        })?;
        if let Some(path) = rel_to_folder(cwd, folder, path) {
            let what = format!("{} {}: {}", result.kind, result.name, result.hints.join(", "));
            hits.push(Hit { path, line: Some(line), col: Some(col), what });
        }
    }
    Ok(hits)
}

/// `location` as its path, line and column, split at its last two `:`; None when either number does not read.
fn place(location: &str) -> Option<(&str, u32, u32)> {
    let (rest, col) = location.rsplit_once(':')?;
    let (path, line) = rest.rsplit_once(':')?;
    Some((path, line.parse().ok()?, col.parse().ok()?))
}
