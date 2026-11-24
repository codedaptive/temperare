//! `detekt_unlisted`: the stop for a detekt check naming something temperare's list does not hold.

use super::detekt_rule_list;
use crate::types::Stop;

/// The detekt release `detekt_rules.txt` was taken from, as its `# version:` header line says.
const LIST_VERSION: &str = "1.23.8";

/// `NotListed` naming the first part of `check`, split on `,`, that is neither a rule nor a rule set of
/// `detekt_rule_list` exactly as written (`NoSuchRule`, `ALL`, `Style`); None when every part is one. `detekt_refusal`
/// has already refused a malformed part, so such a part is a well-formed name tm cannot pick: detekt would run, match
/// nothing and read clean, so the rule is not available instead. `stopped` adds `detekt <version>: `.
pub(crate) fn detekt_unlisted(check: &str) -> Option<Stop> {
    let rules = detekt_rule_list();
    check.split(',').find(|part| !rules.iter().any(|(set, rule)| part == set || part == rule)).map(|part| Stop::NotListed {
        said: format!("{part} is not in temperare's list of detekt codes (taken from detekt {LIST_VERSION})"),
    })
}
