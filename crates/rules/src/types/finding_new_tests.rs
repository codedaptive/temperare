//! ZUT for finding_new.rs: `line` takes a number or an Option (None stored as 0), owner is None, and every other
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
fn a_plain_number_is_stored_as_the_line() {
    assert_eq!(built(7, None).line, 7);
}

#[test]
fn some_number_is_stored_as_the_line() {
    assert_eq!(built(Some(7), None).line, 7);
}

#[test]
fn none_is_stored_as_zero() {
    assert_eq!(built(None, None).line, 0);
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
        line: 7,
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
