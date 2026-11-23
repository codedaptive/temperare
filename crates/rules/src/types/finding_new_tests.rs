//! Unit tests for finding_new.rs: `line` takes a number or an Option, owner is None, and every other
//! argument lands in its field.

use super::{Engine, Finding, Kind, RelPath, Severity};

fn built(line: impl Into<Option<u32>>, col: Option<u32>) -> Finding {
    Finding::new(
        "no-bool",
        (Engine::AstGrep, Kind::Security, Severity::Warning),
        RelPath::new("src/a.rs"),
        line,
        col,
        "stored Bool",
        "Store a bitmap field.",
    )
}

#[test]
fn line_is_stored_as_given() {
    for (case, finding, expected) in [
        ("number", built(7, None), Some(7)),
        ("some", built(Some(7), None), Some(7)),
        ("none", built(None, None), None),
    ] {
        assert_eq!(finding.line, expected, "{case}");
    }
}

#[test]
fn every_field_lands_where_it_should_with_owner_none() {
    let expected = Finding {
        rule: "no-bool".to_string(),
        engine: Engine::AstGrep,
        kind: Kind::Security,
        severity: Severity::Warning,
        owner: None,
        path: RelPath::new("src/a.rs"),
        line: Some(7),
        col: Some(3),
        what: "stored Bool".to_string(),
        fix: "Store a bitmap field.".to_string(),
    };
    assert_eq!(built(7, Some(3)), expected);
}

#[test]
fn col_passes_through() {
    assert_eq!(built(7, Some(3)).col, Some(3));
    assert_eq!(built(7, None).col, None);
}
