//! `parse_ast_grep`: `ast-grep scan --json=stream` output read into hits.

use std::path::Path;

use procio::Output;
use serde::Deserialize;

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::{Hit, ParseError};

/// One match line as ast-grep prints it; fields not read here are ignored.
#[derive(Deserialize)]
struct Match {
    file: String,
    range: Range,
    message: Option<String>,
    #[serde(rename = "ruleId")]
    rule_id: Option<String>,
}

#[derive(Deserialize)]
struct Range {
    start: Position,
}

/// ast-grep's line and column are 0-based.
#[derive(Deserialize)]
struct Position {
    line: u32,
    column: u32,
}

/// The hits in ast-grep's JSON lines, in output order. Every line is a match of the rule file, which is the check.
/// Empty stdout is a clean run when ast-grep exits 0 and not an answer otherwise (a rule it could not read exits
/// non-zero and prints only to stderr). A match whose file is not under `folder` is outside the rule's scope and
/// is skipped.
pub fn parse_ast_grep(out: &Output, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    let stdout = std::str::from_utf8(&out.stdout).map_err(|_| parse_error(out))?;
    if stdout.trim().is_empty() && out.code != Some(0) {
        return Err(parse_error(out));
    }
    let mut hits = Vec::new();
    for line in stdout.lines().filter(|line| !line.trim().is_empty()) {
        let found: Match = serde_json::from_str(line).map_err(|_| parse_error(out))?;
        let Some(path) = rel_to_folder(cwd, folder, &found.file) else { continue };
        let what = found.message.filter(|message| !message.is_empty()).or(found.rule_id).unwrap_or_default();
        hits.push(Hit { path, line: found.range.start.line + 1, col: Some(found.range.start.column + 1), what });
    }
    Ok(hits)
}

/// The error for output that is not an answer, quoting the first 200 bytes of stderr, or of stdout when stderr is
/// empty.
fn parse_error(out: &Output) -> ParseError {
    let quoted = printed_start(out);
    ParseError { why: format!("ast-grep output is not JSON match lines (exit {:?}): {quoted}", out.code) }
}
