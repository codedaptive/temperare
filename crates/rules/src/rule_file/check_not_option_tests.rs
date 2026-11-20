//! Unit tests for check_not_option.rs, with the expected line and column written out.

use super::check_not_option::check_not_option;
use crate::Problem;
use toml::de::DeTable;

const FIX: &str = "A check, config or allow value is never an engine option: remove the leading '-'.";

/// The Problems for the first `[[rule]]` of `"[[rule]]\n" + body`; the body starts on line 2.
fn check(body: &str) -> Vec<Problem> {
    let text = format!("[[rule]]\n{body}");
    let doc = DeTable::parse(&text).unwrap();
    let item = &doc.get_ref().get("rule").unwrap().get_ref().as_array().unwrap()[0];
    let mut out = Vec::new();
    check_not_option(&text, item.get_ref().as_table().unwrap(), &mut out);
    out
}

#[test]
fn a_dashed_check_is_flagged_at_its_string() {
    let out = check("check = \"-x\"\n");
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!((out[0].line, out[0].col), (2, 9));
    assert!(out[0].what.contains("check") && out[0].what.contains("\"-x\""), "{}", out[0].what);
    assert_eq!(out[0].fix, FIX);
}

#[test]
fn a_dashed_config_is_flagged_at_its_string() {
    let out = check("check = \"c\"\nconfig = \"--foo\"\n");
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!((out[0].line, out[0].col), (3, 10));
    assert!(out[0].what.contains("config") && out[0].what.contains("\"--foo\""), "{}", out[0].what);
    assert_eq!(out[0].fix, FIX);
}

#[test]
fn a_dashed_allow_value_is_flagged_at_its_string() {
    let out = check("params = { allow = [\"ok\", \"-bad\"] }\n");
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!((out[0].line, out[0].col), (2, 27));
    assert!(out[0].what.contains("params.allow") && out[0].what.contains("\"-bad\""), "{}", out[0].what);
    assert_eq!(out[0].fix, FIX);
}

#[test]
fn two_dashed_allow_values_are_two_problems() {
    let out = check("params = { allow = [\"-a\", \"-b\"] }\n");
    let at: Vec<_> = out.iter().map(|p| (p.line, p.col)).collect();
    assert_eq!(at, vec![(2, 21), (2, 27)], "{out:?}");
}

#[test]
fn inner_dashes_are_clean() {
    let out = check("check = \"no-stored-bool\"\nconfig = \"a-b.toml\"\nparams = { allow = [\"clippy::x-y\"] }\n");
    assert_eq!(out, vec![]);
}

#[test]
fn an_empty_string_is_clean() {
    assert_eq!(check("check = \"\"\nconfig = \"\"\nparams = { allow = [\"\"] }\n"), vec![]);
}

#[test]
fn a_check_or_config_that_is_not_a_string_is_left_to_the_type_checks() {
    assert_eq!(check("check = [\"-x\"]\nconfig = { a = \"-y\" }\n"), vec![]);
}

#[test]
fn params_that_is_not_a_table_is_left_to_the_type_checks() {
    assert_eq!(check("params = \"-x\"\n"), vec![]);
    assert_eq!(check("params = [\"-x\"]\n"), vec![]);
}

#[test]
fn allow_that_is_not_an_array_is_ignored() {
    assert_eq!(check("params = { allow = \"-x\" }\n"), vec![]);
}

#[test]
fn allow_elements_that_are_not_strings_are_ignored() {
    assert_eq!(check("params = { allow = [1, [\"-x\"], { a = \"-y\" }] }\n"), vec![]);
}

#[test]
fn other_params_keys_are_the_engines() {
    assert_eq!(check("params = { deny = [\"-x\"], flag = \"-y\" }\n"), vec![]);
}
