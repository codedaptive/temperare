//! `override_owner`: how a Problem names the `[[override]]` it is in.

/// `[[override]] for "<rule>"` for an override whose `rule` is a string, else `[[override]]`.
pub(crate) fn override_owner(rule: Option<&str>) -> String {
    rule.map_or_else(|| "[[override]]".to_string(), |rule| format!("[[override]] for \"{rule}\""))
}
