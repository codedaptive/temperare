//! `no_such_rule`: yamllint's words saying it has no rule the rule's check names.

use procio::Output;

/// The stderr line of `out` that is exactly `invalid config: no such rule: "<check>"`; None when there is none.
/// yamllint 1.38.0 given a rule id it lacks prints only that line and exits 255. A line naming another rule is not
/// about the rule's check: it is a name in the repository's own config (a typo there, say), which must not make the
/// rule's check read as unknown, so it is left alone and the run fails on its exit code instead.
pub(crate) fn no_such_rule(out: &Output, check: &str) -> Option<String> {
    let said = format!("invalid config: no such rule: \"{check}\"");
    String::from_utf8_lossy(&out.stderr).lines().find(|line| *line == said).map(str::to_string)
}
