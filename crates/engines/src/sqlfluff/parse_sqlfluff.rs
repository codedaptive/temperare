//! `parse_sqlfluff`: sqlfluff's JSON report, read into the hits of one rule.

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use serde::Deserialize;
use std::path::Path;

/// The codes sqlfluff gives a file it could not parse, template or lex. They are not rule findings: the file was
/// not fully checked, so the rule cannot read clean on it.
const NOT_CHECKED: [&str; 3] = ["PRS", "TMP", "LXR"];

/// One file of `sqlfluff lint --format json`'s array; its `statistics` and `timings` are not read.
#[derive(Deserialize)]
struct File {
    /// As the file was passed, relative to the run's cwd.
    filepath: String,
    violations: Vec<Violation>,
}

/// One violation; its `name`, `warning` flag, `fixes` and end position are not read.
#[derive(Deserialize)]
struct Violation {
    /// 1-based.
    start_line_no: u32,
    /// 1-based.
    start_line_pos: u32,
    code: String,
    description: String,
}

/// The hits in `out`, in report order. Exit 0 and 1 are answers (sqlfluff exits 1 on any violation that is not a
/// config `warnings` one, 0 otherwise); any other exit is a `ParseError` quoting `printed_start`, as is a stdout that is
/// not the JSON array (an unknown dialect in the config exits 1 with stdout empty and a traceback on stderr,
/// fixtures/fixtures.txt; sqlfluff 4.5.0 exits 2 only for `--dialect`, which cctao never passes). A violation with a
/// `NOT_CHECKED` code is a `ParseError` naming its file, line and description. Every other violation is a hit,
/// whether sqlfluff marked it a warning or not: its file made relative to `folder` from `cwd` (both canonical) and
/// dropped when outside it, its line and column, its text "<code>: <description>". `_check` is not read: the command
/// line names only the rule's check, and hits are not filtered by code because a name or group check reports under
/// other codes. A file's own `-- sqlfluff:rules:` or `exclude_rules:` directive can change what runs
/// (`sqlfluff_entry.rs` names them).
pub fn parse_sqlfluff(out: &Output, _check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    if !matches!(out.code, Some(0 | 1)) {
        return Err(ParseError { why: format!("sqlfluff stopped without a report: {}", printed_start(out)) });
    }
    let files: Vec<File> = serde_json::from_slice(&out.stdout)
        .map_err(|_| ParseError { why: format!("sqlfluff printed no JSON report: {}", printed_start(out)) })?;
    let unchecked = files.iter().find_map(|file| {
        file.violations.iter().find(|v| NOT_CHECKED.contains(&v.code.as_str())).map(|v| (file, v))
    });
    if let Some((file, violation)) = unchecked {
        return Err(ParseError {
            why: format!(
                "sqlfluff could not read {}, so it did not check it: line {}: {}: {}",
                file.filepath, violation.start_line_no, violation.code, violation.description
            ),
        });
    }
    Ok(files
        .into_iter()
        .filter_map(|file| Some((rel_to_folder(cwd, folder, &file.filepath)?, file.violations)))
        .flat_map(|(path, violations)| {
            violations.into_iter().map(move |violation| Hit {
                path: path.clone(),
                line: Some(violation.start_line_no),
                col: Some(violation.start_line_pos),
                what: format!("{}: {}", violation.code, violation.description),
            })
        })
        .collect())
}
