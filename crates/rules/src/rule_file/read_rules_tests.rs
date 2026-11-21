//! Unit tests for read_rules.rs.

use super::read_rules::read_rules;
use std::collections::BTreeSet;
use toml::de::DeTable;

const TWO_RULES: &str = "[[rule]]\nid = \"a\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\nparams = { n = 1 }\n\
                         [[rule]]\nid = \"b\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"g\"\n";

#[test]
fn rules_are_read_in_file_order_with_their_params() {
    let doc = DeTable::parse(TWO_RULES).unwrap();
    let mut out = Vec::new();
    let (rules, ids) = read_rules(TWO_RULES, doc.get_ref(), &[], false, &mut out);
    assert_eq!(out, vec![]);
    let read: Vec<&str> = rules.iter().map(|r| r.id.as_str()).collect();
    assert_eq!(read, vec!["a", "b"]);
    assert_eq!(rules[0].params.get("n"), Some(&toml::Value::Integer(1)));
    assert!(rules[1].params.is_empty());
    assert_eq!(ids, BTreeSet::from(["a".to_string(), "b".to_string()]));
}

#[test]
fn a_repeated_id_is_a_problem_at_its_second_value_and_that_rule_is_left_out() {
    let text = "[[rule]]\nid = \"a\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\n\
                [[rule]]\nid = \"a\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"g\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    let (rules, ids) = read_rules(text, doc.get_ref(), &[], false, &mut out);
    assert_eq!(rules.len(), 1);
    assert_eq!(rules[0].check, "c");
    assert_eq!(ids, BTreeSet::from(["a".to_string()]));
    assert_eq!(out.len(), 1);
    assert_eq!((out[0].line, out[0].col), (7, 6));
    assert!(out[0].what.contains("\"a\""), "{}", out[0].what);
}

#[test]
fn ids_include_rules_that_have_problems() {
    let text = "[[rule]]\nid = \"broken\"\nengine = \"clippy\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    let (rules, ids) = read_rules(text, doc.get_ref(), &[], false, &mut out);
    assert!(rules.is_empty());
    assert_eq!(ids, BTreeSet::from(["broken".to_string()]));
    assert_eq!(out.len(), 2, "missing check and fix");
}

#[test]
fn a_set_is_read_as_a_set_each_rule_checked_for_its_tm_id() {
    let text = "[[rule]]\nid = \"tm-a\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\nrequired_params = [\"e\"]\n\
                [[rule]]\nid = \"b\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"g\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    let (rules, ids) = read_rules(text, doc.get_ref(), &[], true, &mut out);
    let read: Vec<(&str, Vec<String>)> = rules.iter().map(|r| (r.id.as_str(), r.required_params.clone())).collect();
    assert_eq!(read, vec![("tm-a", vec!["e".to_string()])]);
    assert_eq!(ids, BTreeSet::from(["tm-a".to_string(), "b".to_string()]));
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!((out[0].line, out[0].col), (8, 6));
    assert_eq!(out[0].what, "baseline rule id \"b\" does not start with \"tm-\"");
}

#[test]
fn an_own_file_is_read_as_an_own_file() {
    let text = "[[rule]]\nid = \"tm-a\"\nengine = \"clippy\"\ncheck = \"c\"\nfix = \"f\"\n";
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    let (rules, _) = read_rules(text, doc.get_ref(), &[], false, &mut out);
    assert_eq!(rules.len(), 0);
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!((out[0].line, out[0].col), (2, 6));
    assert_eq!(out[0].what, "id \"tm-a\" starts with \"tm-\", which is kept for baseline rules");
}
