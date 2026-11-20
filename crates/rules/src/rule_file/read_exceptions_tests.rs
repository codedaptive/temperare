//! Unit tests for read_exceptions.rs.

use super::read_exceptions::read_exceptions;
use crate::{Exception, RelPath};
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
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (2, 8));
    assert!(out[0].what.contains("\"ghost\""), "{}", out[0].what);
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
