//! `parse_pylint`: pylint's json2 output, read into the hits of one rule.

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use serde::Deserialize;
use std::path::Path;

/// The path pylint gives its own messages about its options (W0012, R0022), which are in no file.
const COMMAND_LINE: &str = "Command line";

/// `pylint --output-format=json2`'s one object; its `statistics` are not read.
#[derive(Deserialize)]
struct Report {
    messages: Vec<Message>,
}

/// One message as json2 prints it; other fields (such as type, symbol, module, endLine, absolutePath) are not read.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Message {
    message_id: String,
    message: String,
    path: String,
    line: Option<u32>,
    column: Option<u32>,
}

/// The hits of rule `check` in `out`, in output order: each message whose `messageId` equals `check`. A message on
/// the path "Command line" is pylint's own warning about its options, not a finding, so it is skipped by name
/// (`rel_to_folder` would join it under `cwd` and keep it). Otherwise its path, relative to `cwd`, is made relative
/// to `folder` (a file outside `folder` is not a hit); its line is kept and its column, 0-based in pylint, is made
/// 1-based; a null line or column gives none. Any exit code with such stdout is an answer (pylint's exit is a bit
/// field of the message types it found). stdout that is not that object, pylint's "No files to lint: exiting."
/// included, is a `ParseError` quoting what pylint printed, so the rule is never read as clean.
pub fn parse_pylint(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    let report: Report = serde_json::from_slice(&out.stdout)
        .map_err(|_| ParseError { why: format!("pylint printed no JSON findings: {}", printed_start(out)) })?;
    Ok(report
        .messages
        .into_iter()
        .filter(|message| message.message_id == check && message.path != COMMAND_LINE)
        .filter_map(|message| {
            Some(Hit {
                path: rel_to_folder(cwd, folder, &message.path)?,
                line: message.line,
                col: message.column.map(|column| column + 1),
                what: message.message,
            })
        })
        .collect())
}
