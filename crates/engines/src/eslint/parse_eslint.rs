//! `parse_eslint`: ESLint's JSON output, read into the hits of one rule.

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use serde::Deserialize;
use std::path::Path;

/// One linted file as `eslint --format json` prints it; the counts, source and deprecated rules are not read.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FileResult {
    file_path: String,
    messages: Vec<Message>,
    /// The messages a suppression hid: an `eslint-suppressions.json` entry (or an inline directive, which
    /// `--no-inline-config` turns off).
    suppressed_messages: Vec<Message>,
}

/// One message of a linted file; its fix and suggestions are not read.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Message {
    /// Null for a message that is no rule's, such as a parsing error; it matches no check.
    rule_id: Option<String>,
    /// True when ESLint could not lint the file at all (a parsing error).
    #[serde(default)]
    fatal: bool,
    message: String,
    line: Option<u32>,
    column: Option<u32>,
}

/// The hits of rule `check` in `out`: per file in output order, its `messages` then its `suppressedMessages` whose
/// `ruleId` is `check`. Reading the suppressed ones is how cctao turns `eslint-suppressions.json` off (S-67 (1)): a
/// finding the file hides is still a finding. stdout is one JSON array of files; any exit code with such stdout is an
/// answer (ESLint exits 1 when it reports an error; warnings alone exit 0). `cwd` is where ESLint ran and `folder` the rule file's folder, both
/// canonical; a message in a file outside `folder` is not a hit. A `fatal` message in any file, whatever its rule, is
/// a `ParseError` naming the file (relative to `folder` when it is under it) and ESLint's message: a file ESLint could
/// not parse is never clean. stdout that is not that array is a `ParseError` quoting what ESLint printed.
pub(crate) fn parse_eslint(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    let files: Vec<FileResult> = serde_json::from_slice(&out.stdout)
        .map_err(|_| ParseError { why: format!("eslint printed no JSON findings: {}", printed_start(out)) })?;
    if let Some(why) = files.iter().find_map(|file| fatal_why(file, cwd, folder)) {
        return Err(ParseError { why });
    }
    Ok(files
        .into_iter()
        .flat_map(|file| {
            let path = rel_to_folder(cwd, folder, &file.file_path);
            file.messages
                .into_iter()
                .chain(file.suppressed_messages)
                .filter(|message| message.rule_id.as_deref() == Some(check))
                .filter_map(move |message| {
                    Some(Hit { path: path.clone()?, line: message.line, col: message.column, what: message.message })
                })
        })
        .collect())
}

/// The why for `file`'s first fatal message, if it has one.
fn fatal_why(file: &FileResult, cwd: &Path, folder: &Path) -> Option<String> {
    let message = file.messages.iter().chain(&file.suppressed_messages).find(|message| message.fatal)?;
    let name = rel_to_folder(cwd, folder, &file.file_path)
        .map_or_else(|| file.file_path.clone(), |path| path.as_str().to_string());
    Some(format!("eslint could not parse {name}: {}", message.message))
}
