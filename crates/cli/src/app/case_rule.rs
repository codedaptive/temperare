//! `case_rule`: the rule one baseline case runs, with the params that case gives.

use rules::{missing_param_names, Rule};

/// The rule its failing case (`failing`) or its passing case runs: `rule` with its `params` and the case's own params
/// (`failing_params` or `passing_params`) inserted over them, a key both give taking the case's value, and
/// `not_applicable` cleared. Err with the why when the case's own params lack a required param: the case's own params
/// alone decide whether a required param is given, never a default in `rule.params`; the case then runs on
/// `rule.params` with the case's own over them.
pub(crate) fn case_rule(rule: &Rule, failing: bool) -> Result<Rule, String> {
    let given = if failing { &rule.failing_params } else { &rule.passing_params };
    let missing = missing_param_names(rule, Some(given));
    if !missing.is_empty() {
        return Err(format!("the case gives no value for required params: {}", missing.join(", ")));
    }
    let mut case = rule.clone();
    case.params.extend(given.clone());
    case.not_applicable = None;
    Ok(case)
}
