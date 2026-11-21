//! `baseline_json`: the results of `tm baseline test` as one JSON object.

use super::baseline_rule_result::BaselineRuleResult;
use super::case_verdict::CaseVerdict;
use serde_json::{json, Value};

/// `rules`, each rule in the order given with its set, id, engine, `result` (its verdict) and its failing and passing
/// cases as `{result, why}` (why null when the case holds), and `counts` of rules, holds, broken, not available and
/// not applicable. A result is written `holds`, `broken`, `not_available` or `not_applicable`. The caller adds `exit`
/// and `next`.
pub(crate) fn baseline_json(results: &[BaselineRuleResult]) -> Value {
    let rules: Vec<Value> = results
        .iter()
        .map(|r| {
            json!({
                "set": r.set,
                "rule": r.rule,
                "engine": r.engine.name(),
                "result": json_result(r.verdict()),
                "failing": case_json(&r.failing),
                "passing": case_json(&r.passing),
            })
        })
        .collect();
    let count = |verdict: &str| results.iter().filter(|r| r.verdict() == verdict).count();
    json!({
        "rules": rules,
        "counts": {
            "rules": results.len(),
            "holds": count("holds"),
            "broken": count("broken"),
            "not_available": count("not available"),
            "not_applicable": count("not applicable"),
        },
    })
}

/// A verdict as JSON writes it: `not available` becomes `not_available` and `not applicable` `not_applicable`.
fn json_result(verdict: &str) -> &str {
    match verdict {
        "not available" => "not_available",
        "not applicable" => "not_applicable",
        // `holds` and `broken` are written as they are.
        other => other,
    }
}

/// One case as `{result, why}`.
fn case_json(case: &CaseVerdict) -> Value {
    match case {
        CaseVerdict::Holds => json!({"result": "holds", "why": null}),
        CaseVerdict::Broken { why } => json!({"result": "broken", "why": why}),
        CaseVerdict::NotAvailable { why } => json!({"result": "not_available", "why": why}),
        CaseVerdict::NotApplicable { why } => json!({"result": "not_applicable", "why": why}),
    }
}
