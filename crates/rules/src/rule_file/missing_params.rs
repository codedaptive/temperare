//! `missing_params`: why a baseline rule is not applicable for want of its required params.

use super::missing_param_names::missing_param_names;
use crate::Rule;

/// None when every name in `rule.required_params` is a key of `given`, the params the rule's `[[override]]` gave
/// (None when no override gave params); otherwise the reason the rule is not applicable, naming the missing params in
/// their required order. A default in the set's own `params` never counts: the repository must give each one.
pub(crate) fn missing_params(rule: &Rule, given: Option<&toml::Table>) -> Option<String> {
    let missing = missing_param_names(rule, given);
    if missing.is_empty() {
        return None;
    }
    Some(format!(
        "required params not given: {}; give them in the params of an [[override]] for \"{}\"",
        missing.join(", "),
        rule.id
    ))
}
