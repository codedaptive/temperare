//! `unknown_ruleset`: PMD's words saying a part of the check is no rule or category it has.

use procio::Output;

/// The first line of `out`'s stderr that is `[WARN] No rules found in ruleset <part>` (a rule PMD's category does not
/// hold; PMD exits 0) or starts `[ERROR] Cannot load ruleset <part>:` (a category PMD does not have; exit 1), for a
/// comma part of `check`; None otherwise. A line naming another reference is not the rule's, so the rule is not blamed
/// for it.
pub(crate) fn unknown_ruleset(out: &Output, check: &str) -> Option<String> {
    String::from_utf8_lossy(&out.stderr)
        .lines()
        .find(|line| {
            check.split(',').any(|part| {
                *line == format!("[WARN] No rules found in ruleset {part}")
                    || line.starts_with(&format!("[ERROR] Cannot load ruleset {part}:"))
            })
        })
        .map(str::to_string)
}
