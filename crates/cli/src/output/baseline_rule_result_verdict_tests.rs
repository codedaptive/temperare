//! ZUT for baseline_rule_result_verdict.rs: a rule's verdict for each pair of case verdicts.

use super::baseline_rule_result::BaselineRuleResult;
use super::case_verdict::CaseVerdict;
use rules::Engine;

fn broken() -> CaseVerdict {
    CaseVerdict::Broken { why: "gave no finding".to_string() }
}

fn not_available() -> CaseVerdict {
    CaseVerdict::NotAvailable {
        why: "ruff not found on PATH".to_string(),
        cause: Some(engines::NotAvailableCause::EngineNotOnPath),
    }
}

fn not_applicable() -> CaseVerdict {
    CaseVerdict::NotApplicable { why: "required params not given: entity_types".to_string() }
}

#[test]
fn verdict() {
    let rows = [
        ("both hold", CaseVerdict::Holds, CaseVerdict::Holds, "holds"),
        ("failing broken", broken(), CaseVerdict::Holds, "broken"),
        ("passing broken", CaseVerdict::Holds, broken(), "broken"),
        ("failing not available", not_available(), CaseVerdict::Holds, "not available"),
        ("passing not available", CaseVerdict::Holds, not_available(), "not available"),
        ("broken beside not available", not_available(), broken(), "broken"),
        ("not available beside broken", broken(), not_available(), "broken"),
        ("failing not applicable", not_applicable(), CaseVerdict::Holds, "not applicable"),
        ("not applicable beside not available", not_available(), not_applicable(), "not applicable"),
        ("broken beside not applicable", not_applicable(), broken(), "broken"),
    ];
    for (name, failing, passing, expected) in rows {
        let r = BaselineRuleResult {
            set: "standard@1".to_string(),
            rule: "tm-zeta".to_string(),
            engine: Engine::Ruff,
            failing,
            passing,
        };
        assert_eq!(r.verdict(), expected, "{name}");
    }
}
