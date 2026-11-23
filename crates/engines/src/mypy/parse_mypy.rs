//! `parse_mypy`: mypy's JSON-lines output, read into the hits of one rule.

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use serde::Deserialize;
use std::path::Path;

/// One message as `mypy --output=json` prints it, one per stdout line; `end_line`, `end_column` and `hint` (the
/// follow-on note's text) are not read.
#[derive(Deserialize)]
struct Message {
    file: String,
    /// 1-based; -1 for a message tied to no line (mypy's "Duplicate module named").
    line: i64,
    /// 0-based; -1 when there is none.
    column: i64,
    message: String,
    /// Null for a message that is no error code's.
    code: Option<String>,
    /// "error" or "note" (a `reveal_type` note, say).
    severity: String,
}

/// The hits of rule `check` in `out`, in output order. stdout is one JSON message per non-empty line. Exit 0 and 1
/// are answers (mypy exits 1 when it finds any error). Exit 2 is a blocking error after which mypy checks nothing
/// else (a syntax error, a duplicate module, no Python file), so it is a `ParseError` quoting the first error
/// message as `{file}:{line}: {message}`, or `printed_start` when there is none: reading on would call the rule
/// clean. The one exception is `check` `syntax` with at least one error message on stdout and every error message's
/// code `syntax`: those errors are the rule's findings. A hit is a message with severity "error" and code `check`;
/// its file is made relative to `folder` from `cwd` (both canonical) and dropped when outside it; its line kept when
/// >= 1; its column, 0-based in mypy's JSON, kept 1-based when >= 0; its text the message. A non-empty stdout line
/// that is not such a message is a `ParseError` quoting `printed_start`.
pub fn parse_mypy(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    if let Some(err) = blocking(out, check) {
        return Err(err);
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let messages = stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str::<Message>(line))
        .collect::<Result<Vec<Message>, _>>()
        .map_err(|_| ParseError { why: format!("mypy printed no JSON findings: {}", printed_start(out)) })?;
    Ok(messages
        .into_iter()
        .filter(|message| message.severity == "error" && message.code.as_deref() == Some(check))
        .filter_map(|message| {
            Some(Hit {
                path: rel_to_folder(cwd, folder, &message.file)?,
                line: u32::try_from(message.line).ok().filter(|line| *line >= 1),
                col: u32::try_from(message.column).ok().map(|column| column + 1),
                what: message.message,
            })
        })
        .collect())
}

/// The error for a run mypy stopped at a blocking error (exit 2), quoting the first stdout line that is an error
/// message, or what mypy printed when no line is one; None for any other exit, and for `check` `syntax` when stdout
/// holds error messages and all of them are syntax errors.
fn blocking(out: &Output, check: &str) -> Option<ParseError> {
    if out.code != Some(2) {
        return None;
    }
    let errors: Vec<Message> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .filter_map(|line| serde_json::from_str::<Message>(line).ok())
        .filter(|message| message.severity == "error")
        .collect();
    let syntax_only = !errors.is_empty() && errors.iter().all(|message| message.code.as_deref() == Some("syntax"));
    if check == "syntax" && syntax_only {
        return None;
    }
    let first = errors.first().map_or_else(
        || printed_start(out),
        |message| format!("{}:{}: {}", message.file, message.line, message.message),
    );
    Some(ParseError { why: format!("mypy stopped at a blocking error, so it checked nothing for `{check}`: {first}") })
}
