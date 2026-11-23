//! `unknown_rule`: detekt's words saying the rule's check names a rule the running detekt does not have.

use super::picked_rules;
use procio::Output;

/// The first line of `out`'s stderr, trimmed, that begins `Property '`, says `misspelled or does not exist`, and whose
/// quoted property path is exactly `<set>><Rule>` for a `(set, rule)` that `check` picks (`picked_rules`); None when
/// there is none. tm's layer names only the picked pairs, each rule under its set, so detekt fails its config
/// validation (exit 3) with exactly such a path only when the running detekt lacks a rule from tm's list. Any other
/// path (a pair the check does not pick, a bare set, or a rule with a further part such as an option name) comes from
/// a repository config's own typo: it is left to the reader, which makes the rule `Failed`, so the rule's check is
/// never blamed for it.
pub(crate) fn unknown_rule(out: &Output, check: &str) -> Option<String> {
    let picked: Vec<String> = picked_rules(check).into_iter().map(|(set, rule)| format!("{set}>{rule}")).collect();
    String::from_utf8_lossy(&out.stderr)
        .lines()
        .map(str::trim)
        .find(|line| {
            line.contains("misspelled or does not exist")
                && line
                    .strip_prefix("Property '")
                    .and_then(|rest| rest.split_once('\''))
                    .is_some_and(|(path, _)| picked.iter().any(|pair| pair == path))
        })
        .map(str::to_string)
}
