//! ZUT for baseline_next.rs: the next-step text for each exit code, each expected line a literal.

use super::baseline_next;
use super::baseline_rule_result::BaselineRuleResult;
use super::case_verdict::CaseVerdict;
use rules::Engine;

fn result(rule: &str, engine: Engine, failing: CaseVerdict) -> BaselineRuleResult {
    BaselineRuleResult {
        set: "standard@1".to_string(),
        rule: rule.to_string(),
        engine,
        failing,
        passing: CaseVerdict::Holds,
    }
}

fn not_available(rule: &str, engine: Engine) -> BaselineRuleResult {
    result(rule, engine, CaseVerdict::NotAvailable { why: "not found on PATH".to_string() })
}

fn not_applicable(rule: &str) -> BaselineRuleResult {
    let why = "required params not given: entity_types".to_string();
    result(rule, Engine::Ruff, CaseVerdict::NotApplicable { why })
}

#[test]
fn baseline_next_text() {
    let rows = [
        ("nothing to do", 0, vec![result("tm-zeta", Engine::Ruff, CaseVerdict::Holds)], "nothing to do"),
        (
            "broken",
            1,
            vec![result("tm-zeta", Engine::Ruff, CaseVerdict::Broken { why: "gave no finding".to_string() })],
            "fix each broken rule or its cases above",
        ),
        (
            "each engine once, in first-named order, skipping rules that hold",
            3,
            vec![
                not_available("tm-zeta", Engine::SwiftLint),
                result("tm-mid", Engine::Clippy, CaseVerdict::Holds),
                not_available("tm-alpha", Engine::Ruff),
                not_available("tm-beta", Engine::SwiftLint),
            ],
            "install swiftlint, ruff, then run tm baseline test again",
        ),
        (
            "not applicable only",
            3,
            vec![result("tm-zeta", Engine::Ruff, CaseVerdict::Holds), not_applicable("tm-alpha")],
            "each not-applicable rule above says why it was not run",
        ),
        (
            "not applicable and not available: both parts, install first",
            3,
            vec![not_applicable("tm-zeta"), not_available("tm-alpha", Engine::Clippy)],
            "install clippy, then run tm baseline test again; each not-applicable rule above says why it was not run",
        ),
    ];
    for (name, exit, results, expected) in rows {
        assert_eq!(baseline_next(exit, &results), expected, "{name}");
    }
}
