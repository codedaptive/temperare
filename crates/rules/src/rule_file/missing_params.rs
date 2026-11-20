//! `missing_params`: why a baseline rule is not applicable for want of its required params.

use crate::Rule;

/// None when every name in `rule.required_params` is a key of `given`, the params the rule's `[[override]]` gave
/// (None when no override gave params); otherwise the reason the rule is not applicable, naming the missing params in
/// their required order. A default in the set's own `params` never counts: the repository must give each one.
pub(crate) fn missing_params(rule: &Rule, given: Option<&toml::Table>) -> Option<String> {
    let missing: Vec<&str> = rule
        .required_params
        .iter()
        .filter(|name| !given.is_some_and(|given| given.contains_key(name.as_str())))
        .map(String::as_str)
        .collect();
    if missing.is_empty() {
        return None;
    }
    Some(format!(
        "required params not given: {}; give them in the params of an [[override]] for \"{}\"",
        missing.join(", "),
        rule.id
    ))
}
