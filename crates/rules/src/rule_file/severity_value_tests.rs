//! ZUT for severity_value.rs.

use super::severity_value::severity_value;
use crate::{Problem, Severity};
use toml::de::DeTable;

/// `severity_value` on the whole of `text` as one table.
fn read(text: &str) -> (Option<Severity>, Vec<Problem>) {
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    (severity_value(text, doc.get_ref(), &mut out), out)
}

#[test]
fn an_absent_severity_is_error() {
    let (severity, out) = read("id = \"r\"\n");
    assert_eq!(severity, Some(Severity::Error));
    assert_eq!(out, vec![]);
}

#[test]
fn warning_is_read() {
    let (severity, out) = read("severity = \"warning\"\n");
    assert_eq!(severity, Some(Severity::Warning));
    assert_eq!(out, vec![]);
}

#[test]
fn an_unknown_severity_is_none_with_a_problem_at_the_value() {
    let (severity, out) = read("id = \"r\"\nseverity = \"fatal\"\n");
    assert_eq!(severity, None);
    assert_eq!(
        out,
        vec![Problem {
            line: 2,
            col: 12,
            what: "unknown severity \"fatal\"".to_string(),
            fix: "Use \"error\" or \"warning\".".to_string(),
        }]
    );
}
