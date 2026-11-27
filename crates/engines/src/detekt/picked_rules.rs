//! `picked_rules`: the detekt rules a check picks.

use super::detekt_rule_list;

/// Each `(rule set, rule)` of `detekt_rule_list` that `check` picks, in list order and once each: a part of `check`
/// (split on `,`) that is a rule's name picks that rule, and one that is a rule set's name picks every rule of the set,
/// those that need type resolution included (`typed_why` then makes the check not available). A part that is neither
/// picks nothing; `detekt_unlisted` names it.
pub(crate) fn picked_rules(check: &str) -> Vec<(&'static str, &'static str)> {
    let parts: Vec<&str> = check.split(',').collect();
    detekt_rule_list().into_iter().filter(|(set, rule)| parts.iter().any(|part| part == set || part == rule)).collect()
}
