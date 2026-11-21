//! Unit tests for check_security_owner.rs, with the expected line and column written out.

use super::check_security_owner::check_security_owner;
use crate::Problem;
use toml::de::DeTable;

fn check(text: &str, owner: Option<&str>, baseline_security_at: Option<usize>) -> Vec<Problem> {
    let doc = DeTable::parse(text).unwrap();
    let mut out = Vec::new();
    check_security_owner(text, doc.get_ref(), owner, baseline_security_at, &mut out);
    out
}

/// The one Problem a missing owner gives, at `line`:`col`.
fn missing_owner(line: usize, col: usize) -> Problem {
    Problem {
        line,
        col,
        what: "a security rule needs a top-level `owner`, and the rule file names none".to_string(),
        fix: "Add a top-level `owner = \"<kit>\"`: a security finding must name its owner.".to_string(),
    }
}

/// A quality rule `q`, then security rules `s1` (kind at 9:8) and `s2`; `s1` has no `fix`, a problem elsewhere.
const TEXT: &str = "[[rule]]\nid = \"q\"\nengine = \"ruff\"\ncheck = \"c\"\nfix = \"f\"\n\
                    [[rule]]\nid = \"s1\"\nengine = \"tessera\"\nkind = \"security\"\ncheck = \"c\"\n\
                    [[rule]]\nid = \"s2\"\nengine = \"tessera\"\nkind = \"security\"\ncheck = \"c\"\nfix = \"f\"\n";

/// A file with no rule of its own; its `baseline` entry string starts at byte 24 (line 2, column 13).
const TAKES: &str = "version = 1\nbaseline = [\"sec@1\"]\n";

#[test]
fn no_owner_is_one_problem_at_the_first_security_kind() {
    assert_eq!(check(TEXT, None, None), vec![missing_owner(9, 8)]);
}

#[test]
fn missing_owner_fix_names_a_kit_not_a_zone() {
    let fix = check(TEXT, None, None).into_iter().map(|p| p.fix).collect::<Vec<_>>();
    assert_eq!(fix, vec!["Add a top-level `owner = \"<kit>\"`: a security finding must name its owner.".to_string()]);
}

#[test]
fn an_owner_is_no_problem() {
    assert_eq!(check(TEXT, Some("core"), None), vec![]);
}

#[test]
fn no_security_rule_needs_no_owner() {
    assert_eq!(check("[[rule]]\nid = \"q\"\nkind = \"quality\"\n", None, None), vec![]);
    assert_eq!(check("version = 1\n", None, None), vec![]);
}

#[test]
fn a_baseline_security_rule_with_no_owner_is_the_problem_at_its_entry() {
    assert_eq!(check(TAKES, None, Some(24)), vec![missing_owner(2, 13)]);
}

#[test]
fn an_own_security_rule_comes_before_a_baseline_one() {
    assert_eq!(check(TEXT, None, Some(0)), vec![missing_owner(9, 8)]);
}

#[test]
fn an_owner_covers_a_baseline_security_rule() {
    assert_eq!(check(TAKES, Some("core"), Some(24)), vec![]);
}
