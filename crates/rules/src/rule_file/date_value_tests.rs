//! Unit tests for date_value.rs.

use super::date_value::date_value;
use crate::Date;
use toml::de::DeTable;

#[test]
fn absent_is_none_and_no_problem() {
    let text = "other = \"2026-12-31\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(date_value(text, doc.get_ref(), "expires", &mut out), None);
    assert_eq!(out, vec![]);
}

#[test]
fn a_real_date_comes_back() {
    let text = "expires = \"2026-12-31\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(date_value(text, doc.get_ref(), "expires", &mut out), Date::new(2026, 12, 31));
    assert_eq!(out, vec![]);
}

#[test]
fn an_integer_is_one_wrong_type_problem_at_the_value() {
    // Line 2 is `expires = 20261231`; the value starts at column 11.
    let text = "a = 1\nexpires = 20261231\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(date_value(text, doc.get_ref(), "expires", &mut out), None);
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (2, 11));
    assert_eq!(out[0].what, "`expires` must be a string, not integer");
}

#[test]
fn an_impossible_date_is_one_problem_at_the_opening_quote() {
    // Line 2 is `expires = "2026-02-30"`; the opening quote is column 11.
    let text = "a = 1\nexpires = \"2026-02-30\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(date_value(text, doc.get_ref(), "expires", &mut out), None);
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (2, 11));
    assert_eq!(out[0].what, "`expires` must be a real date written YYYY-MM-DD, not \"2026-02-30\"");
    assert_eq!(out[0].fix, "Write `expires` as a real calendar date, \"YYYY-MM-DD\".");
}

#[test]
fn a_date_without_leading_zeros_is_one_problem_at_the_opening_quote() {
    // Line 1 is `  expires = "2026-2-3"` after two spaces; the opening quote is column 13.
    let text = "  expires = \"2026-2-3\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(date_value(text, doc.get_ref(), "expires", &mut out), None);
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (1, 13));
    assert_eq!(out[0].what, "`expires` must be a real date written YYYY-MM-DD, not \"2026-2-3\"");
    assert_eq!(out[0].fix, "Write `expires` as a real calendar date, \"YYYY-MM-DD\".");
}
