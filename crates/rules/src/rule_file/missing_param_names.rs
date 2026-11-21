//! `missing_param_names`: the required params of a rule that the params given for it lack.

use crate::Rule;

/// The names in `rule.required_params` that are not keys of `given`, in their required order; empty when every one is
/// given. `given` is the params the caller holds for the rule (rules passes the rule's `[[override]]` params), None
/// when there are none, so then every required name is returned. A default in the rule's own `params` never counts: the
/// caller must give each one.
/// Returns names only; each caller words its own reason.
pub fn missing_param_names(rule: &Rule, given: Option<&toml::Table>) -> Vec<String> {
    rule.required_params
        .iter()
        .filter(|name| !given.is_some_and(|given| given.contains_key(name.as_str())))
        .cloned()
        .collect()
}
