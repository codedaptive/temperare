//! Unit tests for string_value.rs.

use super::string_value::string_value;
use toml::de::DeTable;

#[test]
fn a_string_comes_back_with_its_byte_offset() {
    let text = "k = \"x\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(string_value(text, doc.get_ref(), "k", &mut out), Some(("x".to_string(), 4)));
    assert_eq!(out, vec![]);
}

#[test]
fn absent_is_none_and_no_problem() {
    let text = "k = \"x\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(string_value(text, doc.get_ref(), "other", &mut out), None);
    assert_eq!(out, vec![]);
}

#[test]
fn not_a_string_is_none_and_a_problem_at_the_value() {
    let text = "k = 5\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(string_value(text, doc.get_ref(), "k", &mut out), None);
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (1, 5));
    assert!(out[0].what.contains("`k`"), "{}", out[0].what);
}
