//! Unit tests for baseline_json.rs: the whole object, written out by hand.

use super::baseline_json;
use super::baseline_rule_result::BaselineRuleResult;
use super::case_verdict::CaseVerdict;
use rules::Engine;
use serde_json::json;

fn result(set: &str, rule: &str, engine: Engine, failing: CaseVerdict, passing: CaseVerdict) -> BaselineRuleResult {
    BaselineRuleResult { set: set.to_string(), rule: rule.to_string(), engine, failing, passing }
}

#[test]
fn baseline_json_object() {
    let mixed = vec![
        result("standard@1", "tm-zeta", Engine::Ruff, CaseVerdict::Holds, CaseVerdict::Holds),
        result(
            "house@1",
            "tm-alpha",
            Engine::AstGrep,
            CaseVerdict::Broken { why: "gave no finding".to_string() },
            CaseVerdict::Holds,
        ),
        result(
            "house@1",
            "tm-mid",
            Engine::SwiftLint,
            CaseVerdict::Holds,
            CaseVerdict::NotAvailable {
                why: "swiftlint not found on PATH".to_string(),
                cause: Some(engines::NotAvailableCause::EngineNotOnPath),
            },
        ),
        result(
            "house@1",
            "tm-gamma",
            Engine::Ruff,
            CaseVerdict::NotApplicable { why: "params not given".to_string() },
            CaseVerdict::NotApplicable { why: "params not given".to_string() },
        ),
    ];
    let rows = [
        (
            "no rules",
            vec![],
            json!({
                "rules": [],
                "counts": {"rules": 0, "holds": 0, "broken": 0, "not_available": 0, "not_applicable": 0},
            }),
        ),
        (
            "mixed, in the order given",
            mixed,
            json!({
                "rules": [
                    {"set": "standard@1", "rule": "tm-zeta", "engine": "ruff", "result": "holds",
                     "failing": {"result": "holds", "why": null}, "passing": {"result": "holds", "why": null}},
                    {"set": "house@1", "rule": "tm-alpha", "engine": "ast-grep", "result": "broken",
                     "failing": {"result": "broken", "why": "gave no finding"},
                     "passing": {"result": "holds", "why": null}},
                    {"set": "house@1", "rule": "tm-mid", "engine": "swiftlint", "result": "not_available",
                     "failing": {"result": "holds", "why": null},
                     "passing": {"result": "not_available", "why": "swiftlint not found on PATH"}},
                    {"set": "house@1", "rule": "tm-gamma", "engine": "ruff", "result": "not_applicable",
                     "failing": {"result": "not_applicable", "why": "params not given"},
                     "passing": {"result": "not_applicable", "why": "params not given"}},
                ],
                "counts": {"rules": 4, "holds": 1, "broken": 1, "not_available": 1, "not_applicable": 1},
            }),
        ),
    ];
    for (name, results, expected) in rows {
        assert_eq!(baseline_json(&results), expected, "{name}");
    }
}
