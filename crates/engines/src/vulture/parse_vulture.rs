//! `parse_vulture`: vulture's text output, read into the hits of one rule.

use super::VULTURE_KINDS;
use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use std::path::Path;

/// The hits of rule `check` in `out`, in output order: each stdout line `<path>:<line>: <message> (<NN>% confidence)`
/// whose message starts as one of `check`'s in `VULTURE_KINDS`. Its path, relative to `cwd`, is made relative to
/// `folder` (a file outside `folder` is not a hit); its line is kept, vulture gives no column, and its `what` is the
/// message without the confidence. Exit 0 (nothing found) and 3 (findings) are answers; any other exit is a
/// `ParseError`. So is any stderr: with PYTHONWARNINGS=ignore (`run_vulture_with`) vulture writes there only for a file it
/// could not read or parse, so a rule is never read clean, or partly clean, over a file vulture skipped. A line of
/// another shape, or a message no kind starts, is a `ParseError` quoting the line, whatever the check.
pub fn parse_vulture(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    if !matches!(out.code, Some(0 | 3)) {
        let exit = match out.code {
            Some(code) => format!("exited {code}"),
            None => "was stopped by a signal".to_string(),
        };
        return Err(failed(format!("vulture {exit}, not 0 or 3: {}", printed_start(out))));
    }
    if !out.stderr.is_empty() {
        return Err(failed(format!("vulture could not read or parse a file: {}", printed_start(out))));
    }
    let mut hits = Vec::new();
    for line in String::from_utf8_lossy(&out.stdout).lines().filter(|line| !line.is_empty()) {
        let (path, number, what) =
            finding_of(line).ok_or_else(|| failed(format!("vulture printed a line that is not a finding: {line}")))?;
        let code = code_of(what).ok_or_else(|| failed(format!("vulture printed a message of no kind tm knows: {line}")))?;
        if code != check {
            continue;
        }
        if let Some(path) = rel_to_folder(cwd, folder, path) {
            hits.push(Hit { path, line: Some(number), col: None, what: what.to_string() });
        }
    }
    Ok(hits)
}

/// `line`'s path, line number and message: the ` (<NN>% confidence)` suffix is cut, then the path is the text before
/// the first `:<digits>: `. None when either is missing.
fn finding_of(line: &str) -> Option<(&str, u32, &str)> {
    let (rest, confidence) = line.strip_suffix("% confidence)")?.rsplit_once(" (")?;
    if confidence.is_empty() || !confidence.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    rest.match_indices(':').find_map(|(at, _)| {
        let after = &rest[at + 1..];
        let digits = after.bytes().take_while(u8::is_ascii_digit).count();
        let message = after[digits..].strip_prefix(": ")?;
        Some((&rest[..at], after[..digits].parse().ok()?, message))
    })
}

/// The code of the kind whose message starts `what`.
fn code_of(what: &str) -> Option<&'static str> {
    VULTURE_KINDS.iter().find(|(_, starts)| starts.iter().any(|start| what.starts_with(start))).map(|(code, _)| *code)
}

fn failed(why: String) -> ParseError {
    ParseError { why }
}
