//! `parse_vale`: Vale's JSON output, read into the hits of one rule.

use super::vale_errors;
use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

/// One alert as `vale --output=JSON` prints it; its other fields (Action, Match, Severity, ...) are not read.
#[derive(Deserialize)]
struct Alert {
    #[serde(rename = "Check")]
    check: String,
    #[serde(rename = "Line")]
    line: u32,
    /// The alert's first and last column on its line, both 1-based.
    #[serde(rename = "Span")]
    span: (u32, u32),
    #[serde(rename = "Message")]
    message: String,
}

/// The hits of rule `check` in `out`: the alerts whose `Check` is exactly `check`, file by file in path order, each
/// file's in output order. stdout is one JSON object from each path as Vale was given it (relative to `cwd`, where
/// it ran) to its alerts, `{}` when clean; Vale exits 1 when an alert is error level and 0 otherwise, so the exit
/// code is not read. A hit's path is made relative to `folder` (an alert outside it is dropped), its line is the
/// alert's `Line` and its column the start of its `Span`; its text is the `Message`. Any error object on stderr
/// (`vale_errors`) is a `ParseError` naming the first one's Code and Text, even beside findings: Vale then left a
/// file unread (a DITA file without the `dita` program, fixtures/dita.txt). stdout that is not that object is a
/// `ParseError` quoting `printed_start`. Vale's W101 warning lines on stderr are not errors.
pub fn parse_vale(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    if let Some((code, text)) = vale_errors(&out.stderr).into_iter().next() {
        return Err(ParseError { why: format!("vale stopped with {code}: {text}") });
    }
    let files: BTreeMap<String, Vec<Alert>> = serde_json::from_slice(&out.stdout)
        .map_err(|_| ParseError { why: format!("vale printed no JSON findings: {}", printed_start(out)) })?;
    Ok(files
        .into_iter()
        .filter_map(|(path, alerts)| Some((rel_to_folder(cwd, folder, &path)?, alerts)))
        .flat_map(|(path, alerts)| {
            alerts.into_iter().filter(|alert| alert.check == check).map(move |alert| Hit {
                path: path.clone(),
                line: Some(alert.line),
                col: Some(alert.span.0),
                what: alert.message,
            })
        })
        .collect())
}
