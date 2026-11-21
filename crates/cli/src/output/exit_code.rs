//! `exit_code`: tm check's exit code, from its report.

use report::Report;
use rules::{Kind, Severity};

/// The exit code for a report; the first that applies wins (2, usage, is decided before a report exists):
/// 1 a kept quality error finding; 5 an engine failed; 3 a rule was not available; 4 a kept error finding (by now a
/// security one); 6 a rule of the repository's own covers nothing (checked after 1, 5, 3 and 4 have all failed to apply); 0 otherwise, warnings alone included. Reads the lists, never
/// `counts`.
pub fn exit_code(report: &Report) -> u8 {
    let errors = || report.findings.iter().filter(|f| f.severity == Severity::Error);
    if errors().any(|f| f.kind == Kind::Quality) {
        1
    } else if !report.failed.is_empty() {
        5
    } else if !report.not_available.is_empty() {
        3
    } else if errors().next().is_some() {
        4
    } else if !report.covers_nothing.is_empty() {
        6
    } else {
        0
    }
}
