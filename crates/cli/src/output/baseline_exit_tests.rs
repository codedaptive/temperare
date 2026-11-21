//! ZUT for baseline_exit.rs: the exit code for sets of rule verdicts.

use super::baseline_exit;
use super::baseline_rule_result::BaselineRuleResult;
use super::case_verdict::CaseVerdict;
use rules::Engine;

fn result(rule: &str, failing: CaseVerdict) -> BaselineRuleResult {
    BaselineRuleResult {
        set: "standard@1".to_string(),
        rule: rule.to_string(),
        engine: Engine::Ruff,
        failing,
        passing: CaseVerdict::Holds,
    }
}

fn holds(rule: &str) -> BaselineRuleResult {
    result(rule, CaseVerdict::Holds)
}

fn broken(rule: &str) -> BaselineRuleResult {
    result(rule, CaseVerdict::Broken { why: "gave no finding".to_string() })
}

fn not_available(rule: &str) -> BaselineRuleResult {
    result(rule, CaseVerdict::NotAvailable { why: "ruff not found on PATH".to_string() })
}

fn not_applicable(rule: &str) -> BaselineRuleResult {
    result(rule, CaseVerdict::NotApplicable { why: "required params not given: entity_types".to_string() })
}

#[test]
fn baseline_exit_codes() {
    let rows = [
        ("no rules", vec![], 0),
        ("all hold", vec![holds("tm-zeta"), holds("tm-alpha")], 0),
        ("one broken", vec![holds("tm-zeta"), broken("tm-alpha")], 1),
        ("one not available", vec![not_available("tm-zeta"), holds("tm-alpha")], 3),
        ("broken outranks not available", vec![not_available("tm-zeta"), broken("tm-alpha")], 1),
        ("one not applicable", vec![holds("tm-zeta"), not_applicable("tm-alpha")], 3),
        ("broken outranks not applicable", vec![not_applicable("tm-zeta"), broken("tm-alpha")], 1),
    ];
    for (name, results, expected) in rows {
        assert_eq!(baseline_exit(&results), expected, "{name}");
    }
}
