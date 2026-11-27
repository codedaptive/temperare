//! `severity_value`: the `severity` of one `[[rule]]` table.

use super::problem_at::problem_at;
use super::string_value::string_value;
use crate::{Problem, Severity};
use toml::de::DeTable;

/// The rule's `severity`: Error when absent; None (with a Problem) when it is not "error", "warning" or "watch".
pub(crate) fn severity_value(text: &str, table: &DeTable, out: &mut Vec<Problem>) -> Option<Severity> {
    if table.get("severity").is_none() {
        return Some(Severity::Error);
    }
    let (name, at) = string_value(text, table, "severity", out)?;
    let severity = Severity::from_name(&name);
    if severity.is_none() {
        out.push(problem_at(text, at, format!("unknown severity \"{name}\""), "Use \"error\", \"warning\" or \"watch\"."));
    }
    severity
}
