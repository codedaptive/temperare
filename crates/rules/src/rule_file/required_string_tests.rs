//! Unit tests for required_string.rs.

use super::required_string::required_string;
use toml::de::DeTable;

#[test]
fn absent_is_a_problem_at_the_given_table_start() {
    // Byte 6 is the start of line 2.
    let text = "a = 1\nb = 2\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(required_string(text, doc.get_ref(), "fix", "[[rule]]", 6, &mut out), None);
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (2, 1));
    assert_eq!(out[0].what, "[[rule]] is missing `fix`");
}

#[test]
fn present_string_comes_back() {
    let text = "fix = \"do it\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(
        required_string(text, doc.get_ref(), "fix", "[[rule]]", 0, &mut out),
        Some(("do it".to_string(), 6))
    );
    assert_eq!(out, vec![]);
}

#[test]
fn wrong_type_is_a_problem_at_the_value_not_missing() {
    let text = "fix = true\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(required_string(text, doc.get_ref(), "fix", "[[rule]]", 0, &mut out), None);
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (1, 7));
    assert!(out[0].what.contains("boolean"), "{}", out[0].what);
}
