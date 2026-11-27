//! `parse_detekt`: detekt's SARIF report, read into the hits of one rule.

use super::picked_rules;
use crate::printed_start::printed_start;
use crate::sarif::parse_sarif;
use crate::types::{Hit, ParseError};
use procio::Output;
use std::path::Path;

/// The hits of `check` in `out`, shaped by `detekt_output` (the report on stdout, detekt's console on stderr). Exit 0
/// (no issues, or fewer than a config's `maxIssues`) and 2 (issues) are answers: `parse_sarif` reads the report for the
/// SARIF rule ids of the rules `check` picks, `detekt.<set>.<Rule>` each, so a finding of another rule detekt runs by
/// its in-code defaults is dropped. Each hit's `what` is detekt's message text. Any other exit is a `ParseError`
/// quoting the start of what detekt printed: 1 is a usage error or a crash, 3 an invalid config, none a signal, and
/// detekt then writes no report.
pub fn parse_detekt(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    if !matches!(out.code, Some(0 | 2)) {
        let code = out.code.map_or_else(|| "none".to_string(), |code| code.to_string());
        return Err(ParseError { why: format!("detekt gave no answer (exit {code}): {}", printed_start(out)) });
    }
    let ids: Vec<String> = picked_rules(check).into_iter().map(|(set, rule)| format!("detekt.{set}.{rule}")).collect();
    parse_sarif(out, &ids.join(","), cwd, folder)
}
