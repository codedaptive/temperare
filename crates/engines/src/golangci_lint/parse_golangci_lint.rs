//! `parse_golangci_lint`: golangci-lint's JSON output, read into the hits of one rule.

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use serde::Deserialize;
use std::path::Path;

/// The object `golangci-lint run --output.json.path=stdout` prints; its `Report` is not read.
#[derive(Deserialize)]
struct Printed {
    /// Null reads as no issues.
    #[serde(rename = "Issues")]
    issues: Option<Vec<Issue>>,
}

/// One issue; other fields (Severity, SourceLines, Pos.Offset) are not read.
#[derive(Deserialize)]
struct Issue {
    #[serde(rename = "FromLinter")]
    from_linter: String,
    #[serde(rename = "Text")]
    text: String,
    #[serde(rename = "Pos")]
    pos: Pos,
}

/// Where an issue starts: golangci-lint's absolute file name, 1-based line and column, 0 when it gives none.
#[derive(Deserialize)]
struct Pos {
    #[serde(rename = "Filename")]
    filename: String,
    #[serde(rename = "Line")]
    line: u32,
    #[serde(rename = "Column")]
    column: u32,
}

/// The hits of rule `check` in `out`, in output order: the issues whose linter (`FromLinter`) is one of `check`'s
/// comma-separated linter names, the text with its trailing whitespace trimmed (staticcheck's SA1019 ends in a space).
/// Exit 0 (clean) and 1 (issues) are answers; any other exit is a `ParseError`, since golangci-lint then may still
/// print an empty issue list (exit 7 when the folder has no Go module). stdout that is not the JSON object is a
/// `ParseError` too. So is an issue from `typecheck`: the package did not load or compile, so the linters did not
/// analyse it, and its silence is not clean. `cwd` is where golangci-lint ran and `folder` the rule file's folder,
/// both canonical; an issue in a file outside `folder` is not a hit.
pub fn parse_golangci_lint(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    if !matches!(out.code, Some(0 | 1)) {
        let code = out.code.map_or_else(|| "none".to_string(), |code| code.to_string());
        return Err(ParseError { why: format!("golangci-lint gave no answer (exit {code}): {}", printed_start(out)) });
    }
    let printed: Printed = serde_json::from_slice(&out.stdout).map_err(|_| ParseError {
        why: format!("golangci-lint printed no JSON findings: {}", printed_start(out)),
    })?;
    let issues = printed.issues.unwrap_or_default();
    if let Some(issue) = issues.iter().find(|issue| issue.from_linter == "typecheck") {
        return Err(ParseError {
            why: format!("golangci-lint could not load the code, so its linters did not check it: {}", issue.text),
        });
    }
    Ok(issues
        .into_iter()
        .filter(|issue| check.split(',').any(|linter| linter == issue.from_linter))
        .filter_map(|issue| {
            Some(Hit {
                path: rel_to_folder(cwd, folder, &issue.pos.filename)?,
                line: Some(issue.pos.line).filter(|line| *line != 0),
                col: Some(issue.pos.column).filter(|col| *col != 0),
                what: issue.text.trim_end().to_string(),
            })
        })
        .collect())
}
