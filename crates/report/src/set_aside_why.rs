//! `set_aside_why`: the why the report lists for a rule set aside as not applicable.

use rules::Rule;

/// The why of a not-applicable rule whose reason is None; the rule is still listed, since it does not run.
const NO_REASON: &str = "no reason given";

/// The rule's own not-applicable reason, or "no reason given" when it has none. It reads only the rule's own reason
/// and does not check that the rule sits in `not_applicable_rules`: the caller decides which rules are set aside.
pub fn set_aside_why(rule: &Rule) -> &str {
    rule.not_applicable.as_deref().unwrap_or(NO_REASON)
}
