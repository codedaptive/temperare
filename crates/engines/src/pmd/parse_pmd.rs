//! `parse_pmd`: PMD's SARIF log, read into the hits of one rule.

use crate::printed_start::printed_start;
use crate::sarif::parse_sarif;
use crate::types::{Hit, ParseError};
use procio::Output;
use std::path::Path;

/// Exit 0 (no violation) or 4 (violations): every result of `out`'s SARIF log, read by `parse_sarif` with `ALL` in
/// place of `check`, since PMD's rule id is the bare rule name (`UnusedPrivateField`), never the check, and PMD ran
/// only the check's rules. Exit 5: a `ParseError` saying PMD could not process every file, so files went unchecked;
/// its log may hold no result, and a file PMD could not parse must never read clean. Any other exit (1: a ruleset it
/// could not load): a `ParseError` with the exit code and the start of what PMD printed.
pub fn parse_pmd(out: &Output, _check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    match out.code {
        Some(0 | 4) => parse_sarif(out, "ALL", cwd, folder),
        Some(5) => Err(ParseError {
            why: "pmd could not process every file (exit 5), so files went unchecked".to_string(),
        }),
        code => Err(ParseError { why: format!("pmd exited {code:?}: {}", printed_start(out)) }),
    }
}
