//! ZUT for check_set_yaml.rs.

use super::baseline_set::BaselineSet;
use super::check_set_yaml::check_set_yaml;
use crate::Problem;
use toml::de::DeTable;

/// A set that carries `a.yml` only.
const CARRIES_A: BaselineSet =
    BaselineSet { name: "house", version: 2, ready: true, text: "", yaml: &[("a.yml", "rule: a\n")], cases: &[] };
/// The same set carrying nothing.
const CARRIES_NONE: BaselineSet = BaselineSet { yaml: &[], ..CARRIES_A };

const MISSING_A_WHAT: &str = "check \"a.yml\" names a YAML file that baseline set house@2 does not carry";
const MISSING_A_FIX: &str = "Put a.yml in crates/rules/baseline/house/2/ and list it in the set's yaml in \
                             crates/rules/src/rule_file/house_2.rs, or correct check.";

fn check(text: &str, set: &BaselineSet) -> Vec<Problem> {
    let doc = DeTable::parse(text).expect("valid TOML");
    let mut out = Vec::new();
    check_set_yaml(text, doc.get_ref(), set, &mut out);
    out
}

fn rule(engine: &str, check: &str) -> String {
    format!("[[rule]]\nid = \"tm-x\"\nengine = {engine}\ncheck = {check}\nfix = \"f\"\n")
}

#[test]
fn an_ast_grep_rule_naming_a_file_the_set_does_not_carry_is_a_problem_at_its_check() {
    let text = format!("version = 1\n{}", rule("\"ast-grep\"", "\"a.yml\""));
    // Line 5 is `check = "a.yml"`: `check = ` is 8 chars, so the value's opening quote is column 9.
    let expected = Problem { line: 5, col: 9, what: MISSING_A_WHAT.to_string(), fix: MISSING_A_FIX.to_string() };
    assert_eq!(check(&text, &CARRIES_NONE), vec![expected]);
}

#[test]
fn every_missing_file_is_its_own_problem() {
    let text = format!("version = 1\n{}{}", rule("\"ast-grep\"", "\"a.yml\""), rule("\"ast-grep\"", "  \"rs/b.yml\""));
    // The second rule's check is line 10; `check =   ` is 10 chars, so its opening quote is column 11.
    let second = Problem {
        line: 10,
        col: 11,
        what: "check \"rs/b.yml\" names a YAML file that baseline set house@2 does not carry".to_string(),
        fix: "Put rs/b.yml in crates/rules/baseline/house/2/ and list it in the set's yaml in \
              crates/rules/src/rule_file/house_2.rs, or correct check."
            .to_string(),
    };
    let first = Problem { line: 5, col: 9, what: MISSING_A_WHAT.to_string(), fix: MISSING_A_FIX.to_string() };
    assert_eq!(check(&text, &CARRIES_NONE), vec![first, second]);
}

#[test]
fn a_carried_file_is_no_problem() {
    let text = format!("version = 1\n{}", rule("\"ast-grep\"", "\"a.yml\""));
    assert_eq!(check(&text, &CARRIES_A), vec![]);
}

#[test]
fn only_the_rule_naming_an_uncarried_file_is_a_problem() {
    let text = format!("version = 1\n{}{}", rule("\"ast-grep\"", "\"a.yml\""), rule("\"ast-grep\"", "\"c.yml\""));
    // Each rule is 5 lines after `version = 1`, so the second rule's check is line 10; `check = ` is 8 chars, so its
    // opening quote is column 9.
    let expected = Problem {
        line: 10,
        col: 9,
        what: "check \"c.yml\" names a YAML file that baseline set house@2 does not carry".to_string(),
        fix: "Put c.yml in crates/rules/baseline/house/2/ and list it in the set's yaml in \
              crates/rules/src/rule_file/house_2.rs, or correct check."
            .to_string(),
    };
    assert_eq!(check(&text, &CARRIES_A), vec![expected]);
}

#[test]
fn a_rule_of_another_engine_needs_no_carried_file() {
    for engine in ["\"clippy\"", "\"ruff\"", "\"swiftlint\"", "\"tessera\""] {
        let text = format!("version = 1\n{}", rule(engine, "\"a.yml\""));
        assert_eq!(check(&text, &CARRIES_NONE), vec![], "{engine}");
    }
}

#[test]
fn values_of_the_wrong_type_or_an_unknown_engine_give_no_problem_here() {
    let cases = [
        format!("version = 1\n{}", rule("1", "\"a.yml\"")),
        format!("version = 1\n{}", rule("\"astgrep\"", "\"a.yml\"")),
        format!("version = 1\n{}", rule("\"ast-grep\"", "1")),
        format!("version = 1\n{}", rule("\"ast-grep\"", "[\"a.yml\"]")),
        "version = 1\nrule = \"a.yml\"\n".to_string(),
        "version = 1\nrule = [\"a.yml\", 1]\n".to_string(),
        "version = 1\n[[rule]]\nid = \"tm-x\"\nengine = \"ast-grep\"\nfix = \"f\"\n".to_string(),
    ];
    for text in &cases {
        assert_eq!(check(text, &CARRIES_NONE), vec![], "{text}");
    }
}
