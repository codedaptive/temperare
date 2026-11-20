//! Unit tests for check_security_owner.rs, with the expected line and column written out.

use super::check_security_owner::check_security_owner;
use crate::Problem;
use toml::de::DeTable;

fn check(text: &str, owner: Option<&str>) -> Vec<Problem> {
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    check_security_owner(text, doc.get_ref(), owner, &mut out);
    out
}

/// A quality rule `q`, then security rules `s1` (kind at 9:8) and `s2`; `s1` has no `fix`, a problem elsewhere.
const TEXT: &str = "[[rule]]\nid = \"q\"\nengine = \"ruff\"\ncheck = \"c\"\nfix = \"f\"\n\
                    [[rule]]\nid = \"s1\"\nengine = \"tessera\"\nkind = \"security\"\ncheck = \"c\"\n\
                    [[rule]]\nid = \"s2\"\nengine = \"tessera\"\nkind = \"security\"\ncheck = \"c\"\nfix = \"f\"\n";

#[test]
fn no_owner_is_one_problem_at_the_first_security_kind() {
    let out = check(TEXT, None);
    assert_eq!(out.len(), 1, "{out:?}");
    assert_eq!((out[0].line, out[0].col), (9, 8));
    assert!(out[0].what.contains("owner"), "{}", out[0].what);
    assert!(out[0].fix.contains("owner = "), "{}", out[0].fix);
}

#[test]
fn missing_owner_fix_names_a_kit_not_a_zone() {
    let out = check(TEXT, None);
    assert_eq!(out.len(), 1, "{out:?}");
    assert!(out[0].fix.contains("<kit>"), "{}", out[0].fix);
    assert!(!out[0].fix.contains("zone"), "{}", out[0].fix);
}

#[test]
fn an_owner_is_no_problem() {
    assert_eq!(check(TEXT, Some("core")), vec![]);
}

#[test]
fn no_security_rule_needs_no_owner() {
    assert_eq!(check("[[rule]]\nid = \"q\"\nkind = \"quality\"\n", None), vec![]);
    assert_eq!(check("version = 1\n", None), vec![]);
}
