//! `parse_yamllint`: yamllint's parsable output, read into the hits of one rule.

use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use std::path::Path;

/// One line of `-f parsable` output, split: its path, line, column, message and rule id.
struct Line<'a> {
    path: &'a str,
    line: u32,
    col: u32,
    what: &'a str,
    rule: &'a str,
}

/// The hits of rule `check` in `out`, in output order: each stdout line `<path>:<line>:<col>: [<level>] <message>
/// (<rule>)` whose rule is `check`, at `error` or `warning` level alike (the level is the config's, not the finding's
/// worth). Its path, relative to `cwd`, is made relative to `folder` (a file outside `folder` is not a hit); its line
/// and column are kept and its `what` is the message alone. Exit 0 (no error-level finding) and 1 (some) are answers;
/// any other exit (255 for a config yamllint refuses, or a signal) is a `ParseError`. So is any stderr: a base config
/// that cannot be read prints a Python traceback and exits 1, the same code as findings, so stderr is the only sign of
/// it. A line whose rule is `syntax` is a `ParseError` naming its file and line: yamllint could not read that file as
/// YAML, so the check's findings there are incomplete, and a rule is never read partly clean. A line of another shape
/// is a `ParseError` quoting it, whatever the check.
pub fn parse_yamllint(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    if !matches!(out.code, Some(0 | 1)) {
        let exit = match out.code {
            Some(code) => format!("exited {code}"),
            None => "was stopped by a signal".to_string(),
        };
        return Err(failed(format!("yamllint {exit}, not 0 or 1: {}", printed_start(out))));
    }
    if !out.stderr.is_empty() {
        return Err(failed(format!("yamllint wrote to stderr: {}", printed_start(out))));
    }
    let mut hits = Vec::new();
    for text in String::from_utf8_lossy(&out.stdout).lines().filter(|text| !text.is_empty()) {
        let found = line_of(text).ok_or_else(|| failed(format!("yamllint printed a line that is not a finding: {text}")))?;
        if found.rule == "syntax" {
            return Err(failed(format!(
                "yamllint could not read {} as YAML at line {}, so its findings there are incomplete: {}",
                found.path, found.line, found.what
            )));
        }
        if found.rule != check {
            continue;
        }
        if let Some(path) = rel_to_folder(cwd, folder, found.path) {
            hits.push(Hit { path, line: Some(found.line), col: Some(found.col), what: found.what.to_string() });
        }
    }
    Ok(hits)
}

/// `text` split as a parsable line: the rule is the last ` (<id>)`, an id of ASCII lower-case letters, digits and `-`;
/// the path is the text before the first `:<digits>:<digits>: [` (a path may hold `:`), and the level after it is
/// `error` or `warning`. None when any part is missing.
fn line_of(text: &str) -> Option<Line<'_>> {
    let (rest, rule) = text.strip_suffix(')')?.rsplit_once(" (")?;
    if rule.is_empty() || !rule.bytes().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-') {
        return None;
    }
    let (at, line, col, leveled) = rest.match_indices(':').find_map(|(at, _)| {
        let (line, after) = number(&rest[at + 1..])?;
        let (col, after) = number(after)?;
        Some((at, line, col, after.strip_prefix(" [")?))
    })?;
    let what = leveled.strip_prefix("error] ").or_else(|| leveled.strip_prefix("warning] "))?;
    Some(Line { path: &rest[..at], line, col, what, rule })
}

/// The digits that start `text` as a number, and the text after the `:` that must follow them.
fn number(text: &str) -> Option<(u32, &str)> {
    let digits = text.bytes().take_while(u8::is_ascii_digit).count();
    Some((text[..digits].parse().ok()?, text[digits..].strip_prefix(':')?))
}

fn failed(why: String) -> ParseError {
    ParseError { why }
}
