//! Unit tests for render_text.rs: whole expected texts, written out by hand from the text format's contract.

use crate::{render_text, Counts, Failed, NotAvailable, Report, Silenced};
use rules::{Engine, Exception, Finding, Kind, RelPath, Severity};

const ZERO_COUNTS: &str = "0 findings (0 error, 0 warning; 0 security), 0 silenced, 0 exceptions in force, 0 not available, 0 failed\n";

fn empty() -> Report {
    Report {
        findings: Vec::new(),
        silenced: Vec::new(),
        exceptions_in_force: 0,
        not_available: Vec::new(),
        failed: Vec::new(),
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

fn finding(severity: Severity, kind: Kind, owner: Option<&str>, col: Option<u32>) -> Finding {
    Finding {
        rule: "no-unwrap".to_string(),
        engine: Engine::Clippy,
        kind,
        severity,
        owner: owner.map(str::to_string),
        path: RelPath::new("src/main.rs"),
        line: 12,
        col,
        what: "used unwrap".to_string(),
        fix: "return an error".to_string(),
    }
}

fn not_available(engine: Engine) -> NotAvailable {
    NotAvailable { rule: "r1".to_string(), engine, why: "not installed".to_string() }
}

fn failed(code: Option<i32>) -> Failed {
    Failed { rule: "r2".to_string(), engine: Engine::Ruff, code, stderr_tail: "boom".to_string() }
}

#[test]
fn an_empty_report_is_only_the_zero_counts_line() {
    assert_eq!(render_text(&empty()), ZERO_COUNTS);
}

#[test]
fn a_finding_with_col_and_owner_shows_both() {
    let report = Report {
        findings: vec![finding(Severity::Error, Kind::Security, Some("cli"), Some(5))],
        ..empty()
    };
    let expected = "src/main.rs:12:5  error security no-unwrap  owner cli\n  used unwrap\n  fix: return an error\n\n"
        .to_string()
        + ZERO_COUNTS;
    assert_eq!(render_text(&report), expected);
}

#[test]
fn a_finding_without_col_or_owner_leaves_both_out() {
    let report = Report { findings: vec![finding(Severity::Warning, Kind::Quality, None, None)], ..empty() };
    let expected =
        "src/main.rs:12  warning quality no-unwrap\n  used unwrap\n  fix: return an error\n\n".to_string() + ZERO_COUNTS;
    assert_eq!(render_text(&report), expected);
}

#[test]
fn findings_keep_report_order() {
    let report = Report {
        findings: vec![
            finding(Severity::Warning, Kind::Quality, None, None),
            finding(Severity::Error, Kind::Security, None, Some(1)),
        ],
        ..empty()
    };
    let expected = "src/main.rs:12  warning quality no-unwrap\n  used unwrap\n  fix: return an error\n\n\
src/main.rs:12:1  error security no-unwrap\n  used unwrap\n  fix: return an error\n\n"
        .to_string()
        + ZERO_COUNTS;
    assert_eq!(render_text(&report), expected);
}

#[test]
fn a_silenced_finding_is_not_listed() {
    let exception = Exception {
        rule: "no-unwrap".to_string(),
        path: RelPath::new("src/main.rs"),
        what: None,
        reason: "startup only".to_string(),
        approved: "maintainer, 2026-11-17".to_string(),
    };
    let silenced = Silenced { finding: finding(Severity::Error, Kind::Quality, None, None), exception };
    let report = Report { silenced: vec![silenced], ..empty() };
    assert_eq!(render_text(&report), ZERO_COUNTS);
}

#[test]
fn a_not_available_rule_gets_its_line() {
    let report = Report { not_available: vec![not_available(Engine::SwiftLint)], ..empty() };
    assert_eq!(render_text(&report), "not available: r1 (swiftlint): not installed\n".to_string() + ZERO_COUNTS);
}

#[test]
fn a_failed_rule_with_a_code_shows_the_exit_code_and_no_stderr() {
    let report = Report { failed: vec![failed(Some(2))], ..empty() };
    assert_eq!(render_text(&report), "failed: r2 (ruff), exit 2\n".to_string() + ZERO_COUNTS);
}

#[test]
fn a_failed_rule_without_a_code_says_no_exit_code() {
    let report = Report { failed: vec![failed(None)], ..empty() };
    assert_eq!(render_text(&report), "failed: r2 (ruff), no exit code\n".to_string() + ZERO_COUNTS);
}

#[test]
fn not_available_lines_come_before_failed_lines() {
    let report = Report { failed: vec![failed(None)], not_available: vec![not_available(Engine::Ruff)], ..empty() };
    let expected = "not available: r1 (ruff): not installed\nfailed: r2 (ruff), no exit code\n".to_string() + ZERO_COUNTS;
    assert_eq!(render_text(&report), expected);
}

#[test]
fn the_counts_line_carries_each_count_in_its_place() {
    let report = Report {
        exceptions_in_force: 6,
        counts: Counts {
            findings: 9,
            errors: 4,
            warnings: 5,
            quality: 11,
            security: 3,
            silenced: 2,
            not_available: 7,
            failed: 8,
        },
        ..empty()
    };
    assert_eq!(
        render_text(&report),
        "9 findings (4 error, 5 warning; 3 security), 2 silenced, 6 exceptions in force, 7 not available, 8 failed\n"
    );
}

#[test]
fn each_engine_has_its_rule_file_name() {
    let report = Report {
        not_available: vec![
            not_available(Engine::Clippy),
            not_available(Engine::SwiftLint),
            not_available(Engine::Ruff),
            not_available(Engine::AstGrep),
            not_available(Engine::Tessera),
        ],
        ..empty()
    };
    let expected = "not available: r1 (clippy): not installed\n\
not available: r1 (swiftlint): not installed\n\
not available: r1 (ruff): not installed\n\
not available: r1 (ast-grep): not installed\n\
not available: r1 (tessera): not installed\n"
        .to_string()
        + ZERO_COUNTS;
    assert_eq!(render_text(&report), expected);
}
