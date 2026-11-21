//! `parse_shellcheck`: shellcheck's `-f json1` output, read into the hits of one rule.

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use serde::Deserialize;
use std::path::Path;

/// What `shellcheck -f json1` prints on stdout.
#[derive(Deserialize)]
struct Json1 {
    comments: Vec<Comment>,
}

/// One comment as json1 prints it; other fields (endLine, endColumn, fix) are not read. `fix` holds source
/// replacements, not a fix message, and a hit's fix comes from the rule.
#[derive(Deserialize)]
struct Comment {
    /// As shellcheck printed it: the path it was given, relative to where it ran.
    file: String,
    line: u32,
    column: u32,
    level: String,
    /// The number of the code: 2086 for SC2086.
    code: u32,
    message: String,
}

/// The hits of rule `check` (one code, `SC2086`) in `out`, in output order: the comments whose `SC<code>` is `check`.
/// `cwd` is where shellcheck ran and `folder` the rule file's folder, both canonical; a comment on a file outside
/// `folder` is not a hit. Not an answer, so a `ParseError`: an exit other than 0 or 1 (shellcheck exits 2 when it
/// could not read a file, still printing `{"comments":[]}`, and 3 or 4 for a bad option); a stderr line saying it was
/// `unable to read --rcfile`, since the run then checked with settings other than the rule's config; stdout that is
/// not json1; and, unless `check` is itself a parse code (SC1000 to SC1999), an error-level comment with a parse code,
/// since shellcheck could not parse that script and found nothing else in it.
pub fn parse_shellcheck(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    if let Some(error) = not_a_run_of_the_rule(out) {
        return Err(error);
    }
    let json: Json1 = serde_json::from_slice(&out.stdout)
        .map_err(|_| ParseError { why: format!("shellcheck printed no JSON findings: {}", printed_start(out)) })?;
    if !is_parse_code(check) {
        if let Some(comment) = json.comments.iter().find(|c| c.level == "error" && (1000..=1999).contains(&c.code)) {
            return Err(ParseError {
                why: format!(
                    "shellcheck could not parse {}, so it was not checked: SC{} {}",
                    comment.file, comment.code, comment.message
                ),
            });
        }
    }
    Ok(json
        .comments
        .into_iter()
        .filter(|comment| format!("SC{}", comment.code) == check)
        .filter_map(|comment| {
            Some(Hit {
                path: rel_to_folder(cwd, folder, &comment.file)?,
                line: Some(comment.line),
                col: Some(comment.column),
                what: comment.message,
            })
        })
        .collect())
}

/// The error for a run whose output says nothing about the rule: an exit other than 0 or 1, or a config not read.
fn not_a_run_of_the_rule(out: &Output) -> Option<ParseError> {
    if !matches!(out.code, Some(0 | 1)) {
        let exit = out.code.map_or_else(|| "on a signal".to_string(), |code| code.to_string());
        return Some(ParseError { why: format!("shellcheck exited {exit}, which is not an answer: {}", printed_start(out)) });
    }
    let stderr = String::from_utf8_lossy(&out.stderr);
    let line = stderr.lines().find(|line| line.contains("unable to read --rcfile"))?;
    Some(ParseError { why: format!("shellcheck did not read the rule's config, so it checked other settings: {line}") })
}

/// Whether `check` is `SC1000` to `SC1999`, the codes shellcheck gives a script it could not parse.
fn is_parse_code(check: &str) -> bool {
    check.strip_prefix("SC1").is_some_and(|rest| rest.len() == 3 && rest.bytes().all(|b| b.is_ascii_digit()))
}
