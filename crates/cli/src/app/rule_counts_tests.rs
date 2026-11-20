use super::rule_counts::rule_counts;
use crate::RuleCount;
use engines::RuleRun;
use rules::{Engine, RuleOutcome};

#[test]
fn preserves_order_engine_names_and_distinguishes_absent_from_empty_runs() {
    let runs = vec![
        RuleRun { rule: "missing".into(), engine: Engine::Ruff, outcome: RuleOutcome::Findings(vec![]), raw_chars: None },
        RuleRun { rule: "empty".into(), engine: Engine::AstGrep, outcome: RuleOutcome::Findings(vec![]), raw_chars: Some(0) },
        RuleRun { rule: "output".into(), engine: Engine::Ruff, outcome: RuleOutcome::Findings(vec![]), raw_chars: Some(123) },
    ];
    assert_eq!(rule_counts(&runs), vec![
        RuleCount { rule: "missing".into(), engine: "ruff".into(), chars: 0, ran: false },
        RuleCount { rule: "empty".into(), engine: "ast-grep".into(), chars: 0, ran: true },
        RuleCount { rule: "output".into(), engine: "ruff".into(), chars: 123, ran: true },
    ]);
}
