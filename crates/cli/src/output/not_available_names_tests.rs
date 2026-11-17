//! ZUT for not_available_names.rs: rule-file engine names, each once, in first-seen order.

use super::not_available_names::not_available_names;
use report::{Counts, NotAvailable, Report};
use rules::Engine;

fn not_available(rule: &str, engine: Engine) -> NotAvailable {
    NotAvailable { rule: rule.to_string(), engine, why: "not on PATH".to_string() }
}

fn report(not_available: Vec<NotAvailable>) -> Report {
    Report {
        findings: vec![],
        silenced: vec![],
        exceptions_in_force: 0,
        not_available,
        failed: vec![],
        base: None,
        counts: Counts {
            findings: 0,
            errors: 0,
            warnings: 0,
            quality: 0,
            security: 0,
            silenced: 0,
            not_available: 0,
            failed: 0,
        },
    }
}

#[test]
fn two_swiftlint_rules_and_one_ast_grep_name_each_engine_once_in_order() {
    let r = report(vec![
        not_available("s1", Engine::SwiftLint),
        not_available("s2", Engine::SwiftLint),
        not_available("g1", Engine::AstGrep),
    ]);
    assert_eq!(not_available_names(&r), vec!["swiftlint".to_string(), "ast-grep".to_string()]);
}

#[test]
fn every_engine_has_its_rule_file_name() {
    let r = report(vec![
        not_available("a", Engine::Clippy),
        not_available("b", Engine::Ruff),
        not_available("c", Engine::Tessera),
    ]);
    assert_eq!(not_available_names(&r), vec!["clippy".to_string(), "ruff".to_string(), "tessera".to_string()]);
}

#[test]
fn an_empty_report_names_nothing() {
    assert_eq!(not_available_names(&report(vec![])), Vec::<String>::new());
}
