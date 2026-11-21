//! Unit tests for exit_code.rs: every row of the exit-code table alone, and the precedence between them; exit 6 for a
//! rule that covers nothing, below 1, 5, 3 and 4.

use super::exit_code::exit_code;
use report::{CoversNothing, Failed, NotAvailable, Report};
use rules::{Engine, Finding, Kind, RelPath, Severity};

fn finding(kind: Kind, severity: Severity) -> Finding {
    Finding::new("r1", (Engine::Ruff, kind, severity), RelPath::new("a.py"), 3, Some(1), "something", "fix it")
}

fn failed() -> Failed {
    Failed { rule: "r2".to_string(), engine: Engine::Ruff, code: Some(2), stderr_tail: "boom".to_string() }
}

fn not_available() -> NotAvailable {
    NotAvailable { rule: "r3".to_string(), engine: Engine::SwiftLint, why: "swiftlint is not on PATH".to_string() }
}

fn report(findings: Vec<Finding>, failed: Vec<Failed>, not_available: Vec<NotAvailable>) -> Report {
    // Counts left all zero on purpose: exit_code reads the lists, never the counts.
    Report { findings, failed, not_available, ..Default::default() }
}

#[test]
fn a_quality_error_alone_is_1() {
    assert_eq!(exit_code(&report(vec![finding(Kind::Quality, Severity::Error)], vec![], vec![])), 1);
}

#[test]
fn a_failed_engine_alone_is_5() {
    assert_eq!(exit_code(&report(vec![], vec![failed()], vec![])), 5);
}

#[test]
fn a_not_available_rule_alone_is_3() {
    assert_eq!(exit_code(&report(vec![], vec![], vec![not_available()])), 3);
}

#[test]
fn a_security_error_alone_is_4() {
    assert_eq!(exit_code(&report(vec![finding(Kind::Security, Severity::Error)], vec![], vec![])), 4);
}

#[test]
fn an_empty_report_is_0() {
    assert_eq!(exit_code(&report(vec![], vec![], vec![])), 0);
}

#[test]
fn a_quality_error_outranks_a_failed_engine() {
    assert_eq!(exit_code(&report(vec![finding(Kind::Quality, Severity::Error)], vec![failed()], vec![])), 1);
}

#[test]
fn a_failed_engine_outranks_a_not_available_rule() {
    assert_eq!(exit_code(&report(vec![], vec![failed()], vec![not_available()])), 5);
}

#[test]
fn a_not_available_rule_outranks_a_security_error() {
    let r = report(vec![finding(Kind::Security, Severity::Error)], vec![], vec![not_available()]);
    assert_eq!(exit_code(&r), 3);
}

#[test]
fn a_security_error_with_a_quality_warning_is_4() {
    let r = report(
        vec![finding(Kind::Security, Severity::Error), finding(Kind::Quality, Severity::Warning)],
        vec![],
        vec![],
    );
    assert_eq!(exit_code(&r), 4);
}

#[test]
fn a_security_warning_alone_is_0() {
    assert_eq!(exit_code(&report(vec![finding(Kind::Security, Severity::Warning)], vec![], vec![])), 0);
}

#[test]
fn a_quality_warning_alone_is_0() {
    assert_eq!(exit_code(&report(vec![finding(Kind::Quality, Severity::Warning)], vec![], vec![])), 0);
}

#[test]
fn covers_nothing_is_6_below_1_5_3_and_4() {
    let covers = || vec![CoversNothing { rule: "r4".to_string(), engine: Engine::Ruff }];
    let warnings = vec![finding(Kind::Quality, Severity::Warning), finding(Kind::Security, Severity::Warning)];
    let rows: [(&str, Vec<Finding>, Vec<Failed>, Vec<NotAvailable>, u8); 6] = [
        ("covers nothing alone", vec![], vec![], vec![], 6),
        ("quality error", vec![finding(Kind::Quality, Severity::Error)], vec![], vec![], 1),
        ("failed engine", vec![], vec![failed()], vec![], 5),
        ("not available rule", vec![], vec![], vec![not_available()], 3),
        ("security error", vec![finding(Kind::Security, Severity::Error)], vec![], vec![], 4),
        ("only warnings", warnings, vec![], vec![], 6),
    ];
    for (name, findings, failed, not_available, want) in rows {
        let r = Report { covers_nothing: covers(), ..report(findings, failed, not_available) };
        assert_eq!(exit_code(&r), want, "covers nothing with {name}");
    }
}
