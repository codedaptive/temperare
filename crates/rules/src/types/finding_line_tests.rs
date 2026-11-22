//! ZUT for finding_line.rs: the stored optional line reads back unchanged.

use super::{Engine, Finding, Kind, RelPath, Severity};

fn literal(line: Option<u32>) -> Finding {
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
fn line_reads_back_as_stored() {
    for (line, expected) in [(Some(7), Some(7)), (Some(1), Some(1)), (None, None), (Some(0), Some(0))] {
        assert_eq!(literal(line).line(), expected, "line {line:?}");
    }
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

