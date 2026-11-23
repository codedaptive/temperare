//! `parse_phpstan`: PHPStan's JSON output, read into the hits of one rule.

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

/// What `phpstan analyse --error-format=json` prints on stdout; `totals` is not read.
#[derive(Deserialize)]
struct Report {
    /// Keyed by the analysed file's absolute path; a BTreeMap, so files come in key order whatever serde_json's
    /// features.
    files: BTreeMap<String, FileReport>,
    /// Errors that belong to no file, such as a parallel worker that crashed.
    errors: Vec<String>,
}

/// One file's messages; its `errors` count is not read.
#[derive(Deserialize)]
struct FileReport {
    messages: Vec<Message>,
}

/// One finding; `ignorable`, `tip` and `identifier` are not read.
#[derive(Deserialize)]
struct Message {
    message: String,
    /// Null for a finding PHPStan ties to no line.
    line: Option<u32>,
}

/// The suffix PHPStan adds to a file key for a finding in a trait, as in
/// `"/w/src/uses_trait.php (in context of class UsesT)"`.
const IN_CONTEXT: &str = " (in context of ";

/// The hits in `out`: every message is one, since the rule's level already chose them (the check is not read). Hits
/// come in file-key order (byte order of the absolute paths), then in each file's message order. A hit's path is its
/// file key with its last ` (in context of ...)` suffix cut, made relative to `folder` from `cwd` (both canonical); a file
/// outside `folder` gives no hit. Its line is the message's, its column None, its text the message. PHPStan exits 1
/// both when it finds anything and when it fails, so the exit code is not read: the JSON decides. A non-empty top-level
/// `errors` (a crashed worker, say) is a `ParseError` quoting the first one, since the files it lists may be partial;
/// stdout that is not that JSON is a `ParseError` naming phpstan and quoting `printed_start`.
pub fn parse_phpstan(out: &Output, _check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    let report: Report = serde_json::from_slice(&out.stdout)
        .map_err(|_| ParseError { why: format!("phpstan printed no JSON report: {}", printed_start(out)) })?;
    if let Some(first) = report.errors.first() {
        return Err(ParseError { why: format!("phpstan reported an error outside any file: {first}") });
    }
    Ok(report
        .files
        .into_iter()
        .filter_map(|(key, file)| {
            let raw = key.rsplit_once(IN_CONTEXT).map_or(key.as_str(), |(path, _)| path);
            Some((rel_to_folder(cwd, folder, raw)?, file.messages))
        })
        .flat_map(|(path, messages)| {
            messages.into_iter().map(move |message| Hit { path: path.clone(), line: message.line, col: None, what: message.message })
        })
        .collect())
}
