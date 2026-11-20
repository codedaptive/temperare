//! ZUT for bool_value.rs.

use super::bool_value::bool_value;
use crate::Problem;
use toml::de::DeTable;

#[test]
fn absent_is_none_and_no_problem() {
    let text = "on = true\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(bool_value(text, doc.get_ref(), "off", &mut out), None);
    assert_eq!(out, vec![]);
}

#[test]
fn true_comes_back_with_its_byte_offset() {
    // `true` starts at byte 12: "x = 1\n" is 6 bytes, then "off = " is 6 more.
    let text = "x = 1\noff = true\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(bool_value(text, doc.get_ref(), "off", &mut out), Some((true, 12)));
    assert_eq!(out, vec![]);
}

#[test]
fn false_comes_back_as_false() {
    let text = "off = false\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(bool_value(text, doc.get_ref(), "off", &mut out), Some((false, 6)));
    assert_eq!(out, vec![]);
}

#[test]
fn a_string_is_none_and_one_problem_at_the_value() {
    let text = "x = 1\noff = \"yes\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(bool_value(text, doc.get_ref(), "off", &mut out), None);
    assert_eq!(
        out,
        vec![Problem {
            line: 2,
            col: 7,
            what: "`off` must be a boolean, not string".to_string(),
            fix: "Write `off` as a boolean.".to_string(),
        }]
    );
}

#[test]
fn an_integer_is_named_as_an_integer() {
    let text = "off = 1\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(bool_value(text, doc.get_ref(), "off", &mut out), None);
    assert_eq!(
        out,
        vec![Problem {
            line: 1,
            col: 7,
            what: "`off` must be a boolean, not integer".to_string(),
            fix: "Write `off` as a boolean.".to_string(),
        }]
    );
}
