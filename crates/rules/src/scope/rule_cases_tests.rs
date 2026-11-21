//! ZUT for rule_cases.rs.

use crate::{CaseFile, Engine, ExpectedFinding, Rule, RuleCases};

#[test]
fn a_rule_carrying_cases_gives_those_exact_cases() {
    let cases = RuleCases {
        failing: vec![CaseFile { path: "src/lib.rs".to_string(), text: "fn f() { x.unwrap(); }\n" }],
        passing: vec![CaseFile { path: "src/lib.rs".to_string(), text: "fn f() { x?; }\n" }],
        expected: ExpectedFinding { rule_id: "tm-no-unwrap".to_string(), path: "src/lib.rs".to_string(), line: 1 },
    };
    let rule = Rule { embedded_cases: Some(cases), ..Rule::new("tm-no-unwrap", Engine::AstGrep, "no-unwrap.yml") };
    assert_eq!(
        rule.cases(),
        Some(&RuleCases {
            failing: vec![CaseFile { path: "src/lib.rs".to_string(), text: "fn f() { x.unwrap(); }\n" }],
            passing: vec![CaseFile { path: "src/lib.rs".to_string(), text: "fn f() { x?; }\n" }],
            expected: ExpectedFinding {
                rule_id: "tm-no-unwrap".to_string(),
                path: "src/lib.rs".to_string(),
                line: 1,
            },
        })
    );
}

#[test]
fn a_rule_built_by_new_gives_none() {
    let rule = Rule::new("own", Engine::AstGrep, "rules/own.yml");
    assert_eq!(rule.cases(), None);
}
