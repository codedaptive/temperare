//! ZUT for load_set.rs, including the guard that every set built into `tm` loads clean.

use super::baseline_set::BaselineSet;
use super::embedded_sets::EMBEDDED_SETS;
use super::load_set::load_set;
use crate::Problem;

const TWO_RULES: &str = "version = 1\nretired = [\"tm-gone-2\", \"tm-gone-1\"]\n\n[[rule]]\nid = \"tm-b\"\n\
engine = \"clippy\"\ncheck = \"clippy::b\"\nfix = \"Fix b.\"\nrequired_params = [\"types\"]\n\n[[rule]]\n\
id = \"tm-a\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"Fix a.\"\n";

fn set(text: &'static str) -> BaselineSet {
    BaselineSet { name: "test", version: 1, text }
}

fn problem(line: usize, col: usize, what: &str, fix: &str) -> Problem {
    Problem { line, col, what: what.to_string(), fix: fix.to_string() }
}

fn problems(text: &'static str) -> Vec<Problem> {
    load_set(&set(text)).expect_err("the set has a problem")
}

const RETIRED_FIX: &str = "Take the rule out of the set, or out of retired: an id is never reused.";
const PREFIX_FIX: &str = "Start every baseline rule's id with \"tm-\".";

#[test]
fn every_embedded_set_loads_with_no_rules_and_no_retired_ids() {
    for embedded in EMBEDDED_SETS {
        let name = format!("{}@{}", embedded.name, embedded.version);
        let (rules, retired) = load_set(embedded).unwrap_or_else(|p| panic!("{name} has problems: {p:?}"));
        let ids: Vec<String> = rules.into_iter().map(|rule| rule.id).collect();
        assert_eq!((ids, retired), (Vec::<String>::new(), Vec::<String>::new()), "{name}");
    }
}

#[test]
fn the_rules_and_retired_ids_come_back_in_file_order() {
    let (rules, retired) = load_set(&set(TWO_RULES)).expect("valid set");
    let ids: Vec<&str> = rules.iter().map(|rule| rule.id.as_str()).collect();
    assert_eq!(ids, vec!["tm-b", "tm-a"]);
    assert_eq!(rules[0].required_params, vec!["types".to_string()]);
    assert_eq!(rules[1].required_params, Vec::<String>::new());
    assert_eq!(retired, vec!["tm-gone-2".to_string(), "tm-gone-1".to_string()]);
}

#[test]
fn an_unknown_top_level_key_is_a_problem() {
    let fix = "Remove it, or use one of: version, rule, retired.";
    assert_eq!(problems("version = 1\nowner = \"x\"\n"), vec![problem(2, 1, "unknown key `owner` in the top level", fix)]);
}

#[test]
fn a_missing_version_is_a_problem() {
    let expected = problem(1, 1, "`version` is missing", "Add `version = 1` as the first line.");
    assert_eq!(problems("retired = []\n"), vec![expected]);
}

#[test]
fn a_retired_id_without_the_prefix_is_a_problem_at_it() {
    // `retired = [` is 11 chars and `"tm-ok", ` 9 more, so "old" starts at column 21.
    let expected = problem(2, 21, "retired id \"old\" does not start with \"tm-\"", PREFIX_FIX);
    assert_eq!(problems("version = 1\nretired = [\"tm-ok\", \"old\"]\n"), vec![expected]);
}

#[test]
fn a_retired_id_that_is_also_a_rule_is_a_problem_at_it() {
    let text = "version = 1\nretired = [\"tm-a\"]\n[[rule]]\nid = \"tm-a\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"f\"\n";
    let expected = problem(2, 12, "retired id \"tm-a\" is also a rule in this set", RETIRED_FIX);
    assert_eq!(problems(text), vec![expected]);
}

#[test]
fn retired_not_an_array_is_a_problem() {
    let expected = problem(2, 11, "`retired` must be an array of strings, not string", "Write `retired` as an array of strings.");
    assert_eq!(problems("version = 1\nretired = \"tm-x\"\n"), vec![expected]);
}

#[test]
fn a_set_rule_id_without_the_prefix_is_a_problem() {
    let text = "version = 1\n[[rule]]\nid = \"a\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"f\"\n";
    let expected = problem(3, 6, "baseline rule id \"a\" does not start with \"tm-\"", PREFIX_FIX);
    assert_eq!(problems(text), vec![expected]);
}

#[test]
fn problems_come_back_sorted_by_line_and_column() {
    // check_keys reports `colour` (line 3) before the retired id (line 2) is checked.
    let fix = "Remove it, or use one of: version, rule, retired.";
    let expected = vec![
        problem(2, 12, "retired id \"old\" does not start with \"tm-\"", PREFIX_FIX),
        problem(3, 1, "unknown key `colour` in the top level", fix),
    ];
    assert_eq!(problems("version = 1\nretired = [\"old\"]\ncolour = 1\n"), expected);
}

#[test]
fn a_syntax_error_is_one_problem_at_its_position() {
    let found = problems("version = = 1\n");
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!((found[0].line, found[0].col), (1, 11), "{found:?}");
    assert!(found[0].what.starts_with("TOML syntax error: "), "{found:?}");
    assert_eq!(found[0].fix, "Correct the TOML syntax at this position.");
}
