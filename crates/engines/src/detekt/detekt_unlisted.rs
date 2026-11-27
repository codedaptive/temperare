//! `detekt_unlisted`: the stop for a detekt check naming something cctao's list does not hold.

use super::{detekt_rule_list, DETEKT_LIST_VERSION};
use crate::types::Stop;

/// `NotListed` naming the first part of `check`, split on `,`, that is neither a rule nor a rule set of
/// `detekt_rule_list` exactly as written (`NoSuchRule`, `ALL`, `Style`); None when every part is one. `detekt_refusal`
/// has already refused a malformed part, so such a part is a well-formed name cctao cannot pick: detekt would run, match
/// nothing and read clean, so the rule is not available instead. The said text names the detekt release the list was
/// taken from (`DETEKT_LIST_VERSION`). `stopped` adds `detekt <version>: `.
pub(crate) fn detekt_unlisted(check: &str) -> Option<Stop> {
    let rules = detekt_rule_list();
    check.split(',').find(|part| !rules.iter().any(|(set, rule)| part == set || part == rule)).map(|part| Stop::NotListed {
        said: format!("{part} is not in cctao's list of detekt codes (taken from detekt {DETEKT_LIST_VERSION})"),
    })
}
