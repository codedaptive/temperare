//! `baseline_next`: the text after `next: ` for `tm baseline test`.

use super::baseline_rule_result::BaselineRuleResult;

/// What the user does next for `exit`: 0 has nothing to do; 1 fixes the broken rules or their cases; 3 gives the
/// parts that apply, joined by `; `: install each engine of a not-available rule, once each, in the order `results`
/// first names it, and read why each not-applicable rule was not run. `baseline_exit` gives no other code; any other
/// points at the output above.
pub(crate) fn baseline_next(exit: u8, results: &[BaselineRuleResult]) -> String {
    match exit {
        0 => "nothing to do".to_string(),
        1 => "fix each broken rule or its cases above".to_string(),
        3 => not_run_parts(results).join("; "),
        _ => "see the output above".to_string(),
    }
}

/// The parts of exit 3's next text: the engines to install when a rule is not available, then the pointer at the
/// not-applicable rules' whys when one is not applicable.
fn not_run_parts(results: &[BaselineRuleResult]) -> Vec<String> {
    let mut parts = Vec::new();
    let engines = engines_to_install(results);
    if !engines.is_empty() {
        parts.push(format!("install {}, then run tm baseline test again", engines.join(", ")));
    }
    if results.iter().any(|r| r.verdict() == "not applicable") {
        parts.push("each not-applicable rule above says why it was not run".to_string());
    }
    parts
}

/// The engine names of the not-available rules, each once, in first-seen order.
fn engines_to_install(results: &[BaselineRuleResult]) -> Vec<&'static str> {
    let mut names = Vec::new();
    for r in results.iter().filter(|r| r.verdict() == "not available") {
        if !names.contains(&r.engine.name()) {
            names.push(r.engine.name());
        }
    }
    names
}
