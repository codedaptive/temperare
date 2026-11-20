//! Unit tests for string_list.rs.

use super::string_list::string_list;
use toml::de::DeTable;

#[test]
fn strings_come_back_with_offsets_and_each_non_string_is_a_problem() {
    // `k = [` is 5 bytes: "a" at 5, 2 at 10, "b" at 13.
    let text = "k = [\"a\", 2, \"b\"]\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    let list = string_list(text, doc.get_ref(), "k", &mut out);
    assert_eq!(list, vec![("a".to_string(), 5), ("b".to_string(), 13)]);
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (1, 11));
}

#[test]
fn absent_is_empty_and_no_problem() {
    let text = "x = 1\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(string_list(text, doc.get_ref(), "k", &mut out), vec![]);
    assert_eq!(out, vec![]);
}

#[test]
fn not_an_array_is_empty_and_a_problem_at_the_value() {
    let text = "k = \"src/**\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(string_list(text, doc.get_ref(), "k", &mut out), vec![]);
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (1, 5));
    assert!(out[0].what.contains("array"), "{}", out[0].what);
}
