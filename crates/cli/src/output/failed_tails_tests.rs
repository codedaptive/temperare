//! ZUT for failed_tails.rs: each failed rule's stderr tail as indented text, in report order.

use super::failed_tails::failed_tails;
use report::{Counts, Failed, Report};
use rules::Engine;

fn failed(rule: &str, engine: Engine, stderr_tail: &str) -> Failed {
    Failed { rule: rule.to_string(), engine, code: Some(2), stderr_tail: stderr_tail.to_string() }
}

fn report(failed: Vec<Failed>) -> Report {
    Report {
        findings: vec![],
        silenced: vec![],
        exceptions_in_force: 0,
        not_available: vec![],
        failed,
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
fn no_failed_rule_is_the_empty_string() {
    assert_eq!(failed_tails(&report(vec![])), "");
}

#[test]
fn a_two_line_tail_is_a_head_line_then_each_line_indented() {
    let r = report(vec![failed("g1", Engine::AstGrep, "error: bad pattern\n  at line 3")]);
    assert_eq!(failed_tails(&r), "stderr of g1 (ast-grep):\n  error: bad pattern\n    at line 3\n");
}

#[test]
fn an_empty_tail_says_empty_on_the_head_line() {
    let r = report(vec![failed("s1", Engine::SwiftLint, "")]);
    assert_eq!(failed_tails(&r), "stderr of s1 (swiftlint): (empty)\n");
}

#[test]
fn two_failed_rules_come_in_report_order() {
    let r = report(vec![failed("r2", Engine::Ruff, "boom"), failed("c1", Engine::Clippy, "")]);
    assert_eq!(failed_tails(&r), "stderr of r2 (ruff):\n  boom\nstderr of c1 (clippy): (empty)\n");
}
