use super::rule_counts::rule_counts;
use crate::RuleCount;
use engines::RuleRun;
use rules::{Engine, RuleOutcome};

#[test]
fn preserves_order_engine_names_and_distinguishes_absent_from_empty_runs() {
    let runs = vec![
        RuleRun::new("missing".into(), Engine::Ruff, RuleOutcome::Findings(vec![]), None),
        RuleRun::new("empty".into(), Engine::AstGrep, RuleOutcome::Findings(vec![]), Some(0)),
        RuleRun::new("output".into(), Engine::Ruff, RuleOutcome::Findings(vec![]), Some(123)),
    ];
    assert_eq!(rule_counts(&runs), vec![
        RuleCount { rule: "missing".into(), engine: "ruff".into(), chars: 0, ran: false },
        RuleCount { rule: "empty".into(), engine: "ast-grep".into(), chars: 0, ran: true },
        RuleCount { rule: "output".into(), engine: "ruff".into(), chars: 123, ran: true },
    ]);
}
