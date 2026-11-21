//! Unit tests for read_exceptions.rs.

use super::read_exceptions::read_exceptions;
use crate::{Date, Exception, Problem, RelPath};
use std::collections::BTreeSet;
use toml::de::DeTable;

fn ids() -> BTreeSet<String> {
    BTreeSet::from(["no-bool".to_string()])
}

#[test]
fn an_exception_is_read_with_its_path_made_relative() {
    let text = "[[exception]]\nrule = \"no-bool\"\npath = \"./src\\\\a.rs\"\nwhat = \"isOn\"\n\
                reason = \"legacy\"\napproved = \"bob 2026-11-17\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    let read = read_exceptions(text, doc.get_ref(), &ids(), &mut out);
    assert_eq!(out, vec![]);
    let expected = Exception {
        rule: "no-bool".to_string(),
        path: RelPath::new("src/a.rs"),
        what: Some("isOn".to_string()),
        expires: None,
        reason: "legacy".to_string(),
        approved: "bob 2026-11-17".to_string(),
    };
    assert_eq!(read, vec![expected]);
}

#[test]
fn an_exception_for_an_unknown_rule_is_a_problem_at_the_value_and_left_out() {
    let text = "[[exception]]\nrule = \"ghost\"\npath = \"a.rs\"\nreason = \"r\"\napproved = \"a\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(read_exceptions(text, doc.get_ref(), &ids(), &mut out), vec![]);
    let expected = Problem {
        line: 2,
        col: 8,
        what: "exception for unknown rule \"ghost\": no rule of this file or of its baseline sets has that id".to_string(),
        fix: "Name the id of a rule in this file or in a set it takes in baseline, or remove the exception.".to_string(),
    };
    assert_eq!(out, vec![expected]);
}

#[test]
fn absent_is_no_exceptions() {
    let text = "version = 1\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(read_exceptions(text, doc.get_ref(), &ids(), &mut out), vec![]);
    assert_eq!(out, vec![]);
}

const EMPTY_WHAT_FIX: &str = "empty what; omit the key to match every finding of the rule at this path";

#[test]
fn an_empty_what_is_a_problem_at_the_value_and_the_exception_is_left_out() {
    let text = "[[exception]]\nrule = \"no-bool\"\npath = \"a.rs\"\nwhat = \"\"\nreason = \"r\"\napproved = \"a\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(read_exceptions(text, doc.get_ref(), &ids(), &mut out), vec![]);
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (4, 8));
    assert_eq!(out[0].fix, EMPTY_WHAT_FIX);
}

#[test]
fn a_whitespace_only_what_is_a_problem_and_the_exception_is_left_out() {
    let text = "[[exception]]\nrule = \"no-bool\"\npath = \"a.rs\"\nwhat = \"   \"\nreason = \"r\"\napproved = \"a\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(read_exceptions(text, doc.get_ref(), &ids(), &mut out), vec![]);
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (4, 8));
    assert_eq!(out[0].fix, EMPTY_WHAT_FIX);
}

#[test]
fn an_absent_what_is_read_as_none_without_a_problem() {
    let text = "[[exception]]\nrule = \"no-bool\"\npath = \"a.rs\"\nreason = \"r\"\napproved = \"a\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    let read = read_exceptions(text, doc.get_ref(), &ids(), &mut out);
    assert_eq!(out, vec![]);
    let expected = Exception {
        rule: "no-bool".to_string(),
        path: RelPath::new("a.rs"),
        what: None,
        expires: None,
        reason: "r".to_string(),
        approved: "a".to_string(),
    };
    assert_eq!(read, vec![expected]);
}

#[test]
fn an_empty_what_and_an_unknown_rule_are_both_reported() {
    let text = "[[exception]]\nrule = \"ghost\"\npath = \"a.rs\"\nwhat = \"\"\nreason = \"r\"\napproved = \"a\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    assert_eq!(read_exceptions(text, doc.get_ref(), &ids(), &mut out), vec![]);
    let at: Vec<(usize, usize)> = out.iter().map(|p| (p.line, p.col)).collect();
    assert_eq!(at, vec![(2, 8), (4, 8)]);
    assert_eq!(out[1].fix, EMPTY_WHAT_FIX);
}

/// An exception for "no-bool" at "a.rs" whose sixth line is `expires = <value>`.
fn with_expires(value: &str) -> String {
    format!("[[exception]]\nrule = \"no-bool\"\npath = \"a.rs\"\nreason = \"r\"\napproved = \"a\"\nexpires = {value}\n")
}

#[test]
fn an_exception_with_a_real_expires_date_is_read() {
    let text = with_expires("\"2026-12-31\"");
    let doc = DeTable::parse(&text).unwrap();
    let mut out = Vec::new();
    let read = read_exceptions(&text, doc.get_ref(), &ids(), &mut out);
    assert_eq!(out, vec![]);
    assert_eq!(read, vec![Exception { expires: Date::new(2026, 12, 31), ..Exception::new("no-bool", "a.rs", "r", "a") }]);
}

#[test]
fn an_expires_that_is_not_a_string_is_one_problem_at_the_value_and_left_out() {
    let text = with_expires("20261231");
    let doc = DeTable::parse(&text).unwrap();
    let mut out = Vec::new();
    assert_eq!(read_exceptions(&text, doc.get_ref(), &ids(), &mut out), vec![]);
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!((out[0].line, out[0].col), (6, 11));
    assert_eq!(out[0].what, "`expires` must be a string, not integer");
}

#[test]
fn a_malformed_expires_is_one_problem_at_the_value_and_left_out() {
    let text = with_expires("\"2026-2-03\"");
    let doc = DeTable::parse(&text).unwrap();
    let mut out = Vec::new();
    assert_eq!(read_exceptions(&text, doc.get_ref(), &ids(), &mut out), vec![]);
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!((out[0].line, out[0].col), (6, 11));
    assert_eq!(out[0].what, "`expires` must be a real date written YYYY-MM-DD, not \"2026-2-03\"");
}

#[test]
fn an_impossible_expires_date_is_one_problem_at_the_value_and_left_out() {
    let text = with_expires("\"2026-02-30\"");
    let doc = DeTable::parse(&text).unwrap();
    let mut out = Vec::new();
    assert_eq!(read_exceptions(&text, doc.get_ref(), &ids(), &mut out), vec![]);
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!((out[0].line, out[0].col), (6, 11));
    assert_eq!(out[0].what, "`expires` must be a real date written YYYY-MM-DD, not \"2026-02-30\"");
}
