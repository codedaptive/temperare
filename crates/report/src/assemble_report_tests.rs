//! ZUT for assemble_report.rs: order, the new-only filter before exceptions, counts, and the carried lists.

use crate::assemble_report;
use procio::Oid;
use rules::{Engine, Exception, Finding, Kind, RelPath, RuleFile, RuleOutcome, Severity};
use std::path::PathBuf;

fn finding(rule: &str, path: &str, line: u32, col: Option<u32>) -> Finding {
    Finding {
        rule: rule.to_string(),
        engine: Engine::Clippy,
        kind: Kind::Quality,
        severity: Severity::Error,
        owner: None,
        path: RelPath::new(path),
        line,
        col,
        what: "violation".to_string(),
        fix: "Fix it.".to_string(),
    }
}

fn graded(kind: Kind, severity: Severity, line: u32) -> Finding {
    Finding { kind, severity, ..finding("r", "a.rs", line, None) }
}

fn exception(rule: &str, path: &str) -> Exception {
    Exception {
        rule: rule.to_string(),
        path: RelPath::new(path),
        what: None,
        reason: "approved use".to_string(),
        approved: "Bob, 2026-11-17".to_string(),
    }
}

fn file(exceptions: Vec<Exception>) -> RuleFile {
    RuleFile { folder: PathBuf::from("."), kits: Vec::new(), owner: None, rules: Vec::new(), exceptions }
}

fn of(findings: Vec<Finding>) -> Vec<(String, RuleOutcome)> {
    vec![("ignored".to_string(), RuleOutcome::Findings(findings))]
}

fn places(findings: &[Finding]) -> Vec<(&str, u32, Option<u32>, &str)> {
    findings.iter().map(|f| (f.path.as_str(), f.line, f.col, f.rule.as_str())).collect()
}

#[test]
fn findings_come_out_sorted_by_path_line_col_rule() {
    let shuffled = vec![
        finding("b", "src/b.rs", 1, None),
        finding("z", "src/a.rs", 2, Some(1)),
        finding("a", "src/a.rs", 2, Some(1)),
        finding("a", "src/a.rs", 2, None),
        finding("a", "src/a.rs", 10, None),
        finding("a", "src/a.rs", 2, Some(0)),
    ];
    let report = assemble_report(&file(vec![]), of(shuffled), None, |f| f);
    assert_eq!(
        places(&report.findings),
        vec![
            ("src/a.rs", 2, None, "a"),
            ("src/a.rs", 2, Some(0), "a"),
            ("src/a.rs", 2, Some(1), "a"),
            ("src/a.rs", 2, Some(1), "z"),
            ("src/a.rs", 10, None, "a"),
            ("src/b.rs", 1, None, "b"),
        ]
    );
}

#[test]
fn a_finding_keep_new_drops_is_neither_silenced_nor_counted() {
    let rules = file(vec![exception("old", "a.rs"), exception("new", "a.rs")]);
    let findings = vec![finding("old", "a.rs", 1, None), finding("new", "a.rs", 2, None), finding("x", "a.rs", 3, None)];
    let keep_new = |found: Vec<Finding>| found.into_iter().filter(|f| f.line != 1).collect();
    let report = assemble_report(&rules, of(findings), None, keep_new);
    assert_eq!(places(&report.findings), vec![("a.rs", 3, None, "x")]);
    let silenced: Vec<_> = report.silenced.iter().map(|s| (s.finding.rule.as_str(), s.exception.rule.as_str())).collect();
    assert_eq!(silenced, vec![("new", "new")]);
    assert_eq!((report.counts.findings, report.counts.silenced), (1, 1));
}

#[test]
fn silenced_findings_are_sorted_by_their_finding() {
    let rules = file(vec![exception("r", "a.rs"), exception("r", "b.rs")]);
    let findings = vec![finding("r", "b.rs", 1, None), finding("r", "a.rs", 9, None), finding("r", "a.rs", 4, None)];
    let report = assemble_report(&rules, of(findings), None, |f| f);
    let lines: Vec<_> = report.silenced.iter().map(|s| (s.finding.path.as_str(), s.finding.line)).collect();
    assert_eq!(lines, vec![("a.rs", 4), ("a.rs", 9), ("b.rs", 1)]);
}

#[test]
fn counts_are_by_severity_and_kind() {
    let findings = vec![
        graded(Kind::Security, Severity::Warning, 1),
        graded(Kind::Quality, Severity::Error, 2),
        graded(Kind::Security, Severity::Error, 3),
    ];
    let counts = assemble_report(&file(vec![]), of(findings), None, |f| f).counts;
    assert_eq!((counts.findings, counts.errors, counts.warnings), (3, 2, 1));
    assert_eq!((counts.quality, counts.security), (1, 2));
}

#[test]
fn exceptions_in_force_is_carried_with_nothing_silenced() {
    let rules = file(vec![exception("a", "x.rs"), exception("b", "y.rs")]);
    let report = assemble_report(&rules, of(vec![finding("c", "z.rs", 1, None)]), None, |f| f);
    assert_eq!(report.exceptions_in_force, 2);
    assert!(report.silenced.is_empty());
    assert_eq!(report.counts.silenced, 0);
}

#[test]
fn not_available_and_failed_are_carried_sorted_with_a_twenty_line_tail() {
    let stderr: String = (1..=25).map(|n| format!("e{n}\n")).collect();
    let outcomes = vec![
        ("x".to_string(), RuleOutcome::Failed { rule: "lint".to_string(), engine: Engine::Clippy, code: None, stderr }),
        ("x".to_string(), RuleOutcome::NotAvailable { rule: "sec".to_string(), engine: Engine::Tessera, why: "absent".to_string() }),
        ("x".to_string(), RuleOutcome::NotAvailable { rule: "fmt".to_string(), engine: Engine::Ruff, why: "gone".to_string() }),
        ("x".to_string(), RuleOutcome::Failed { rule: "ast".to_string(), engine: Engine::AstGrep, code: Some(2), stderr: "boom\n".to_string() }),
    ];
    let report = assemble_report(&file(vec![]), outcomes, None, |f| f);
    let not_available: Vec<_> = report.not_available.iter().map(|n| (n.rule.as_str(), n.engine, n.why.as_str())).collect();
    assert_eq!(not_available, vec![("fmt", Engine::Ruff, "gone"), ("sec", Engine::Tessera, "absent")]);
    let failed: Vec<_> = report.failed.iter().map(|f| (f.rule.as_str(), f.code)).collect();
    assert_eq!(failed, vec![("ast", Some(2)), ("lint", None)]);
    assert_eq!(report.failed[0].stderr_tail, "boom");
    let tail: Vec<&str> = report.failed[1].stderr_tail.lines().collect();
    assert_eq!((tail.len(), tail[0], tail[19]), (20, "e6", "e25"));
    assert_eq!((report.counts.not_available, report.counts.failed, report.counts.findings), (2, 2, 0));
}

#[test]
fn the_base_is_carried() {
    let oid = Oid::parse("0123456789abcdef0123456789abcdef01234567");
    assert_eq!(assemble_report(&file(vec![]), Vec::new(), oid.clone(), |f| f).base, oid);
}

#[test]
fn the_same_input_gives_equal_reports() {
    let rules = file(vec![exception("r", "a.rs")]);
    let input = || of(vec![finding("r", "a.rs", 1, None), finding("q", "b.rs", 2, Some(3)), finding("q", "a.rs", 7, None)]);
    assert_eq!(assemble_report(&rules, input(), None, |f| f), assemble_report(&rules, input(), None, |f| f));
}
