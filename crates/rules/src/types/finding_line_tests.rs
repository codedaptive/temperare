//! Unit tests for finding_line.rs: a stored line reads back as Some, 0 (what `Finding::new` stores for None) as None.

use super::{Engine, Finding, Kind, RelPath, Severity};

fn literal(line: u32) -> Finding {
    Finding {
        rule: "r".to_string(),
        engine: Engine::Clippy,
        kind: Kind::Quality,
        severity: Severity::Error,
        owner: None,
        path: RelPath::new("a.rs"),
        line,
        col: None,
        what: "w".to_string(),
        fix: "f".to_string(),
    }
}

#[test]
fn line_seven_reads_back_as_some_seven() {
    assert_eq!(literal(7).line(), Some(7));
}

#[test]
fn line_one_reads_back_as_some_one() {
    assert_eq!(literal(1).line(), Some(1));
}

#[test]
fn built_with_none_reads_back_as_none() {
    let finding = Finding::new(
        "r",
        (Engine::Ruff, Kind::Quality, Severity::Error),
        RelPath::new("a.py"),
        None,
        None,
        "w",
        "f",
    );
    assert_eq!(finding.line(), None);
}

#[test]
fn a_literal_with_line_zero_reads_back_as_none() {
    assert_eq!(literal(0).line(), None);
}
