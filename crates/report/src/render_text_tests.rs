//! Unit tests for render_text.rs: whole expected texts, written out by hand from the text format's contract.

use crate::{
    render_text, Counts, CoversNothing, EngineVersion, Expired, Failed, NotApplicable, NotAvailable, Report, Silenced,
};
use rules::{Engine, Exception, Finding, Kind, RelPath, Severity};

const ZERO_COUNTS: &str = "0 findings (0 error, 0 warning; 0 security), 0 silenced, 0 exceptions in force, 0 not available, 0 failed, 0 not applicable, 0 covers nothing, 0 expired\n";

fn empty() -> Report {
    Report::default()
}

fn finding(severity: Severity, kind: Kind, owner: Option<&str>, col: Option<u32>) -> Finding {
    finding_at(12, severity, kind, owner, col)
}

fn finding_at(
    line: impl Into<Option<u32>>,
    severity: Severity,
    kind: Kind,
    owner: Option<&str>,
    col: Option<u32>,
) -> Finding {
    let mut finding = Finding::new(
        "no-unwrap",
        (Engine::Clippy, kind, severity),
        RelPath::new("src/main.rs"),
        line,
        col,
        "used unwrap",
        "return an error",
    );
    finding.owner = owner.map(str::to_string);
    finding
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
fn a_finding_with_no_line_prints_line_zero() {
    let report = Report {
        findings: vec![
            finding_at(None, Severity::Error, Kind::Quality, None, Some(5)),
            finding_at(None, Severity::Warning, Kind::Quality, None, None),
        ],
        ..empty()
    };
    let expected = "src/main.rs:0:5  error quality no-unwrap\n  used unwrap\n  fix: return an error\n\n\
src/main.rs:0  warning quality no-unwrap\n  used unwrap\n  fix: return an error\n\n"
        .to_string()
        + ZERO_COUNTS;
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
    let exception = Exception::new("no-unwrap", "src/main.rs", "startup only", "maintainer, 2026-11-17");
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
            not_applicable: 10,
            covers_nothing: 12,
            expired: 13,
        },
        ..empty()
    };
    assert_eq!(
        render_text(&report),
        "9 findings (4 error, 5 warning; 3 security), 2 silenced, 6 exceptions in force, 7 not available, 8 failed, 10 not applicable, 12 covers nothing, 13 expired\n"
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

fn ran(engine: Engine, version: Option<&str>) -> EngineVersion {
    EngineVersion { engine, version: version.map(str::to_string) }
}

#[test]
fn the_engines_line_lists_each_engine_and_version_in_report_order() {
    let engines = vec![ran(Engine::AstGrep, Some("0.39.1")), ran(Engine::Ruff, Some("0.6.0"))];
    let report = Report { engines, ..empty() };
    assert_eq!(render_text(&report), "engines: ast-grep 0.39.1, ruff 0.6.0\n".to_string() + ZERO_COUNTS);
}

#[test]
fn the_engines_line_is_not_sorted() {
    let engines = vec![ran(Engine::Ruff, Some("0.6.0")), ran(Engine::AstGrep, Some("0.39.1"))];
    let report = Report { engines, ..empty() };
    assert_eq!(render_text(&report), "engines: ruff 0.6.0, ast-grep 0.39.1\n".to_string() + ZERO_COUNTS);
}

#[test]
fn an_engine_without_a_version_says_version_unknown() {
    let report = Report { engines: vec![ran(Engine::Clippy, None)], ..empty() };
    assert_eq!(render_text(&report), "engines: clippy (version unknown)\n".to_string() + ZERO_COUNTS);
}

#[test]
fn an_empty_engine_list_prints_no_engines_line() {
    let report = Report { engines: Vec::new(), ..empty() };
    assert_eq!(render_text(&report), ZERO_COUNTS);
}

#[test]
fn the_engines_line_comes_after_failed_lines_and_before_counts() {
    let report = Report {
        failed: vec![failed(Some(2))],
        engines: vec![ran(Engine::Ruff, Some("0.6.0")), ran(Engine::Clippy, None)],
        ..empty()
    };
    let expected =
        "failed: r2 (ruff), exit 2\nengines: ruff 0.6.0, clippy (version unknown)\n".to_string() + ZERO_COUNTS;
    assert_eq!(render_text(&report), expected);
}

fn not_applicable(rule: &str, engine: Engine) -> NotApplicable {
    NotApplicable { rule: rule.to_string(), engine, why: "no swift files".to_string() }
}

#[test]
fn a_not_applicable_rule_is_only_counted() {
    let report = Report {
        not_applicable: vec![not_applicable("r3", Engine::SwiftLint)],
        counts: Counts { not_applicable: 1, ..Counts::default() },
        ..empty()
    };
    assert_eq!(
        render_text(&report),
        "0 findings (0 error, 0 warning; 0 security), 0 silenced, 0 exceptions in force, 0 not available, 0 failed, 1 not applicable, 0 covers nothing, 0 expired\n"
    );
}

fn covers_nothing(rule: &str, engine: Engine) -> CoversNothing {
    CoversNothing { rule: rule.to_string(), engine }
}

#[test]
fn a_covers_nothing_rule_gets_its_line() {
    let report = Report { covers_nothing: vec![covers_nothing("r4", Engine::AstGrep)], ..empty() };
    assert_eq!(render_text(&report), "covers nothing: r4 (ast-grep)\n".to_string() + ZERO_COUNTS);
}

#[test]
fn covers_nothing_lines_keep_report_order() {
    let report = Report {
        covers_nothing: vec![covers_nothing("zz", Engine::Ruff), covers_nothing("aa", Engine::Clippy)],
        ..empty()
    };
    let expected = "covers nothing: zz (ruff)\ncovers nothing: aa (clippy)\n".to_string() + ZERO_COUNTS;
    assert_eq!(render_text(&report), expected);
}

#[test]
fn covers_nothing_lines_come_after_failed_lines_and_before_the_engines_line() {
    let report = Report {
        failed: vec![failed(Some(2))],
        covers_nothing: vec![covers_nothing("r4", Engine::AstGrep)],
        not_applicable: vec![not_applicable("r3", Engine::SwiftLint)],
        engines: vec![ran(Engine::Ruff, Some("0.6.0"))],
        counts: Counts { failed: 1, not_applicable: 2, covers_nothing: 3, ..Counts::default() },
        ..empty()
    };
    let expected = "failed: r2 (ruff), exit 2\ncovers nothing: r4 (ast-grep)\n\
engines: ruff 0.6.0\n\
0 findings (0 error, 0 warning; 0 security), 0 silenced, 0 exceptions in force, 0 not available, 1 failed, 2 not applicable, 3 covers nothing, 0 expired\n";
    assert_eq!(render_text(&report), expected);
}

fn expired(rule: &str, path: &str) -> Expired {
    Expired {
        rule: rule.to_string(),
        path: RelPath::new(path),
        what: None,
        expires: "2026-11-30".to_string(),
        reason: "startup only".to_string(),
        approved: "maintainer, 2026-11-17".to_string(),
    }
}

#[test]
fn an_expired_exception_gets_its_line() {
    let report = Report { expired: vec![expired("no-unwrap", "src/main.rs")], ..empty() };
    assert_eq!(render_text(&report), "expired: no-unwrap src/main.rs (expired 2026-11-30)\n".to_string() + ZERO_COUNTS);
}

#[test]
fn expired_lines_come_after_covers_nothing_lines_and_before_the_engines_line() {
    let report = Report {
        covers_nothing: vec![covers_nothing("r4", Engine::AstGrep)],
        not_applicable: vec![not_applicable("r3", Engine::SwiftLint)],
        expired: vec![expired("zz", "b.rs"), expired("aa", "a.rs")],
        engines: vec![ran(Engine::Ruff, Some("0.6.0"))],
        counts: Counts { not_applicable: 4, covers_nothing: 5, expired: 6, ..Counts::default() },
        ..empty()
    };
    let expected = "covers nothing: r4 (ast-grep)\n\
expired: zz b.rs (expired 2026-11-30)\n\
expired: aa a.rs (expired 2026-11-30)\n\
engines: ruff 0.6.0\n\
0 findings (0 error, 0 warning; 0 security), 0 silenced, 0 exceptions in force, 0 not available, 0 failed, 4 not applicable, 5 covers nothing, 6 expired\n";
    assert_eq!(render_text(&report), expected);
}
