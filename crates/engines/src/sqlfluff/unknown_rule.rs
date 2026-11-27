//! `unknown_rule`: sqlfluff's words saying it does not know the rule's check.

use procio::Output;

/// The marker sqlfluff 4.5.0 puts before the Python list of the `--rules` references it does not know, as in
/// "WARNING    Tried to allowlist unknown rule references: ['NOPE99']". It then lints with no rule and exits 0, so
/// without this the rule would read clean.
const UNKNOWN: &str = "Tried to allowlist unknown rule references: ";

/// The first line of `out`'s stderr whose bracketed list after `UNKNOWN` holds `'<check>'` exactly, trimmed; None
/// when no line does. A line naming only other references is not about the check, so it is left to the reader.
pub(crate) fn unknown_rule(out: &Output, check: &str) -> Option<String> {
    let quoted = format!("'{check}'");
    String::from_utf8_lossy(&out.stderr)
        .lines()
        .find(|line| {
            line.split_once(UNKNOWN).is_some_and(|(_, list)| {
                list.trim().trim_start_matches('[').trim_end_matches(']').split(',').any(|item| item.trim() == quoted)
            })
        })
        .map(|line| line.trim().to_string())
}
