//! `unknown_rule_id`: gitleaks' words saying a rule id in the check is no rule it knows.

use procio::Output;

/// The first line of `out`'s stderr that is exactly "Requested rule <part> not found in rules" for a comma part of
/// `check`; None otherwise. stderr is as `run_gitleaks` leaves it (error lines, time and level gone). A line naming
/// another id (one a repo's config enables, say) is not about the check, so the rule is not blamed for it.
pub(crate) fn unknown_rule_id(out: &Output, check: &str) -> Option<String> {
    String::from_utf8_lossy(&out.stderr)
        .lines()
        .find(|line| check.split(',').any(|part| *line == format!("Requested rule {part} not found in rules")))
        .map(str::to_string)
}
