//! `parse_clippy`: `cargo clippy --message-format=json` output read into hits for one lint, or for every warning.

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use serde::Deserialize;
use std::path::Path;

/// One JSON line cargo prints; only the fields read here.
#[derive(Deserialize)]
struct Record {
    reason: String,
    message: Option<Message>,
    success: Option<bool>,
}

#[derive(Deserialize)]
struct Message {
    message: String,
    /// cargo always prints it; absent reads as "", which no check treats as a warning.
    #[serde(default)]
    level: String,
    code: Option<Code>,
    spans: Vec<Span>,
}

#[derive(Deserialize)]
struct Code {
    code: String,
}

#[derive(Deserialize)]
struct Span {
    file_name: String,
    line_start: u32,
    column_start: u32,
    is_primary: bool,
}

/// The `check` lint's messages in `out`'s stdout, in output order, each at its primary span. `check = "warnings"`
/// takes every warning-level message that has a code, its `what` prefixed `"<code>: "` so it names the lint; any other
/// `check` takes the messages whose code is `check`, at any level, `what` as printed. With `--all-targets` cargo prints
/// a message once per target that compiles its file, so a hit equal to an earlier one is dropped. `cwd` is the folder
/// span paths are read relative to: the workspace root, which is where cargo prints them from even when it ran in a
/// member crate (`hits_clippy` passes it). `folder` is the rule file's folder; a message with no primary span, or one
/// outside `folder`, is skipped. A run is an answer only when a `build-finished` record says it succeeded: a
/// failed build has not linted everything, so it is a `ParseError`, as are a missing record and a non-JSON line.
pub fn parse_clippy(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    let mut hits = Vec::new();
    let mut finished = None;
    for line in String::from_utf8_lossy(&out.stdout).lines().filter(|line| !line.trim().is_empty()) {
        let record: Record = serde_json::from_str(line).map_err(|_| fail(out, "output is not JSON lines"))?;
        match record.reason.as_str() {
            "build-finished" => finished = Some(record.success == Some(true)),
            "compiler-message" => hits.extend(
                record.message.and_then(|message| hit(message, check, cwd, folder)).filter(|found| !hits.contains(found)),
            ),
            _ => {}
        }
    }
    match finished {
        Some(true) => Ok(hits),
        Some(false) => Err(fail(out, "the build failed")),
        None => Err(fail(out, "no build-finished record")),
    }
}

/// The hit for `message` when `check` takes it (a warning with a code under `"warnings"`, else the `check` lint) and
/// it has a primary span under `folder`.
fn hit(message: Message, check: &str, cwd: &Path, folder: &Path) -> Option<Hit> {
    let code = message.code?.code;
    let what = match check {
        "warnings" if message.level == "warning" => format!("{code}: {}", message.message),
        "warnings" => return None,
        _ if code == check => message.message,
        _ => return None,
    };
    let span = message.spans.into_iter().find(|span| span.is_primary)?;
    Some(Hit {
        path: rel_to_folder(cwd, folder, &span.file_name)?,
        line: span.line_start,
        col: Some(span.column_start),
        what,
    })
}

/// A `ParseError` naming clippy and `what` went wrong, quoting the first 200 bytes of stderr, or of stdout when
/// stderr is empty.
fn fail(out: &Output, what: &str) -> ParseError {
    ParseError { why: format!("clippy: {what}: {:?}", printed_start(out)) }
}
