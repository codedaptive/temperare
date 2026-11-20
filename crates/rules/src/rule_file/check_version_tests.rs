//! Unit tests for check_version.rs.

use super::check_version::check_version;
use crate::Problem;
use toml::de::DeTable;

fn check(text: &str) -> Vec<Problem> {
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    check_version(text, doc.get_ref(), &mut out);
    out
}

#[test]
fn integer_one_is_no_problem_in_any_radix() {
    assert_eq!(check("version = 1\n"), vec![]);
    assert_eq!(check("version = 0x1\n"), vec![]);
}

#[test]
fn absent_is_a_problem_at_1_1() {
    let out = check("\n\nother = 1\n");
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (1, 1));
    assert!(out[0].what.contains("version"), "{}", out[0].what);
}

#[test]
fn not_the_integer_one_is_a_problem_at_the_value() {
    for text in ["version = 2\n", "version = \"1\"\n", "version = 1.0\n"] {
        let out = check(text);
        assert_eq!(out.len(), 1, "{text}");
        assert_eq!((out[0].line, out[0].col), (1, 11), "{text}");
        assert!(out[0].what.contains("version"), "{}", out[0].what);
    }
}
