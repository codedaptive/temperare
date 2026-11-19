//! Unit tests for counts_of.rs.

use crate::Counts;
use rules::{Engine, Finding, Kind, RelPath, Severity};

fn finding(kind: Kind, severity: Severity) -> Finding {
    Finding {
        rule: "r".to_string(),
        engine: Engine::Ruff,
        kind,
        severity,
        owner: None,
        path: RelPath::new("a.py"),
        line: 1,
        col: None,
        what: "w".to_string(),
        fix: "f".to_string(),
    }
}

#[test]
fn findings_are_counted_by_severity_and_by_kind() {
    let findings = vec![
        finding(Kind::Quality, Severity::Error),
        finding(Kind::Security, Severity::Error),
        finding(Kind::Security, Severity::Warning),
        finding(Kind::Security, Severity::Error),
    ];
    let expected = Counts {
        findings: 4,
        errors: 3,
        warnings: 1,
        quality: 1,
        security: 3,
        silenced: 5,
        not_available: 6,
        failed: 7,
    };
    assert_eq!(Counts::of(&findings, 5, 6, 7), expected);
}

#[test]
fn no_findings_count_zero() {
    let expected = Counts {
        findings: 0,
        errors: 0,
        warnings: 0,
        quality: 0,
        security: 0,
        silenced: 1,
        not_available: 0,
        failed: 2,
    };
    assert_eq!(Counts::of(&[], 1, 0, 2), expected);
}
