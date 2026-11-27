//! ZUT for load_set.rs, including the guard that every set built into `cctao` loads clean.

use super::baseline_set::BaselineSet;
use super::embedded_sets::EMBEDDED_SETS;
use super::load_set::load_set;
use crate::{CaseFile, ExpectedFinding, Problem, RuleCases};

const TWO_RULES: &str = "version = 1\nretired = [\"tm-gone-2\", \"tm-gone-1\"]\n\n[[rule]]\nid = \"tm-b\"\n\
engine = \"clippy\"\ncheck = \"clippy::b\"\nfix = \"Fix b.\"\nrequired_params = [\"types\"]\n\
failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n\n[[rule]]\n\
id = \"tm-a\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"Fix a.\"\n\
failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n";

/// The case files of the set rule `tm-a`: its failing and its passing case are each the one file `a.rs`.
const CASES_A: &[(&str, &str)] = &[("cases/tm-a/failing/a.rs", "x\n"), ("cases/tm-a/passing/a.rs", "x\n")];

/// The set `test@1` with `text`, carrying `cases` and no YAML.
fn set(text: &'static str, cases: &'static [(&'static str, &'static str)]) -> BaselineSet {
    BaselineSet { name: "test", version: 1, ready: true, text, yaml: &[], cases }
}

fn problem(line: usize, col: usize, what: &str, fix: &str) -> Problem {
    Problem { line, col, what: what.to_string(), fix: fix.to_string() }
}

/// The Problems of `test@1` with `text`, carrying no case files.
fn problems(text: &'static str) -> Vec<Problem> {
    problems_carrying(text, &[])
}

fn problems_carrying(text: &'static str, cases: &'static [(&'static str, &'static str)]) -> Vec<Problem> {
    load_set(&set(text, cases)).expect_err("the set has a problem")
}

const RETIRED_FIX: &str = "Take the rule out of the set, or out of retired: an id is never reused.";
const PREFIX_FIX: &str = "Start every baseline rule's id with \"tm-\".";

/// Each set's rules are its own tests' to check (house_1_tests, house_2_tests, standard_1_tests).
#[test]
fn every_embedded_set_loads_clean_with_no_retired_ids() {
    for embedded in EMBEDDED_SETS {
        let name = format!("{}@{}", embedded.name, embedded.version);
        let (_, retired) = load_set(embedded).unwrap_or_else(|p| panic!("{name} has problems: {p:?}"));
        assert_eq!(retired, Vec::<String>::new(), "{name}");
    }
}

#[test]
fn the_rules_and_retired_ids_come_back_in_file_order() {
    let cases = &[
        ("cases/tm-b/failing/a.rs", "x\n"),
        ("cases/tm-b/passing/a.rs", "x\n"),
        ("cases/tm-a/failing/a.rs", "x\n"),
        ("cases/tm-a/passing/a.rs", "x\n"),
    ];
    let (rules, retired) = load_set(&set(TWO_RULES, cases)).expect("valid set");
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
    let text = "version = 1\nretired = [\"tm-a\"]\n[[rule]]\nid = \"tm-a\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"f\"\n\
                failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n";
    let expected = problem(2, 12, "retired id \"tm-a\" is also a rule in this set", RETIRED_FIX);
    assert_eq!(problems_carrying(text, CASES_A), vec![expected]);
}

#[test]
fn retired_not_an_array_is_a_problem() {
    let expected = problem(2, 11, "`retired` must be an array of strings, not string", "Write `retired` as an array of strings.");
    assert_eq!(problems("version = 1\nretired = \"tm-x\"\n"), vec![expected]);
}

#[test]
fn a_set_rule_id_without_the_prefix_is_a_problem() {
    let text = "version = 1\n[[rule]]\nid = \"a\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"f\"\n\
                failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n";
    let expected = problem(3, 6, "baseline rule id \"a\" does not start with \"tm-\"", PREFIX_FIX);
    let cases = &[("cases/a/failing/a.rs", "x\n"), ("cases/a/passing/a.rs", "x\n")];
    assert_eq!(problems_carrying(text, cases), vec![expected]);
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
fn an_ast_grep_rule_whose_yaml_the_set_does_not_carry_is_a_problem_sorted_with_the_others() {
    let text = "version = 1\ncolour = 1\nretired = [\"old\"]\n[[rule]]\nid = \"tm-a\"\nengine = \"ast-grep\"\n\
                check = \"a.yml\"\nfix = \"f\"\n\
                failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n";
    let unknown_fix = "Remove it, or use one of: version, rule, retired.";
    // `retired = [` is 11 chars, so "old" starts at column 12; `check = ` is 8 chars, so "a.yml" starts at column 9.
    let expected = vec![
        problem(2, 1, "unknown key `colour` in the top level", unknown_fix),
        problem(3, 12, "retired id \"old\" does not start with \"tm-\"", PREFIX_FIX),
        problem(
            7,
            9,
            "check \"a.yml\" names a YAML file that baseline set test@1 does not carry",
            "Put a.yml in crates/rules/baseline/test/1/ and list it in the set's yaml in \
             crates/rules/src/rule_file/test_1.rs, or correct check.",
        ),
    ];
    assert_eq!(problems_carrying(text, CASES_A), expected);
}

/// An ast-grep rule and a clippy rule naming the same carried path.
const CARRIED: BaselineSet = BaselineSet {
    name: "test",
    version: 1,
    ready: true,
    text: "version = 1\n[[rule]]\nid = \"tm-a\"\nengine = \"ast-grep\"\ncheck = \"rust/a.yml\"\nfix = \"f\"\n\
           failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n\
           [[rule]]\nid = \"tm-c\"\nengine = \"clippy\"\ncheck = \"rust/a.yml\"\nfix = \"f\"\n\
           failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
    yaml: &[("rust/a.yml", "id: a\nlanguage: rust\nrule:\n  pattern: $X.unwrap()\n")],
    cases: &[
        ("cases/tm-a/failing/a.rs", "x\n"),
        ("cases/tm-a/passing/a.rs", "x\n"),
        ("cases/tm-c/failing/a.rs", "x\n"),
        ("cases/tm-c/passing/a.rs", "x\n"),
    ],
};

#[test]
fn an_ast_grep_rule_comes_back_with_the_carried_yaml_and_a_clippy_rule_with_none() {
    let (rules, _) = load_set(&CARRIED).unwrap_or_else(|p| panic!("valid set, but: {p:?}"));
    assert_eq!(rules[0].id, "tm-a");
    assert_eq!(rules[0].embedded_yaml(), Some("id: a\nlanguage: rust\nrule:\n  pattern: $X.unwrap()\n"));
    assert_eq!(rules[0].check, "rust/a.yml");
    assert_eq!(rules[1].id, "tm-c");
    assert_eq!(rules[1].embedded_yaml(), None);
}

/// Two rules with cases of their own: `tm-a`'s failing case is two files and finds line 3 of its three-line lib.rs;
/// `tm-b`'s finds line 7 of its seven-line b.rs.
const CASED: BaselineSet = BaselineSet {
    name: "test",
    version: 1,
    ready: true,
    text: "version = 1\n[[rule]]\nid = \"tm-a\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"f\"\n\
           failing = { files = [\"src/lib.rs\", \"Cargo.toml\"], path = \"src/lib.rs\", line = 3 }\n\
           passing = { files = [\"src/lib.rs\"] }\n\
           [[rule]]\nid = \"tm-b\"\nengine = \"clippy\"\ncheck = \"clippy::b\"\nfix = \"f\"\n\
           failing = { files = [\"b.rs\"], path = \"b.rs\", line = 7 }\npassing = { files = [\"b.rs\"] }\n",
    yaml: &[],
    cases: &[
        ("cases/tm-b/passing/b.rs", "b passing\n"),
        ("cases/tm-a/failing/src/lib.rs", "a failing\nlib\nfinding\n"),
        ("cases/tm-a/failing/Cargo.toml", "a failing toml\n"),
        ("cases/tm-a/passing/src/lib.rs", "a passing lib\n"),
        ("cases/tm-b/failing/b.rs", "b\nfailing\n3\n4\n5\n6\nfinding\n"),
    ],
};

#[test]
fn every_rule_comes_back_with_its_own_cases() {
    let (rules, _) = load_set(&CASED).unwrap_or_else(|p| panic!("valid set, but: {p:?}"));
    let file = |path: &str, text: &'static str| CaseFile { path: path.to_string(), text };
    let expected = vec![
        (
            "tm-a".to_string(),
            Some(RuleCases {
                failing: vec![file("src/lib.rs", "a failing\nlib\nfinding\n"), file("Cargo.toml", "a failing toml\n")],
                passing: vec![file("src/lib.rs", "a passing lib\n")],
                expected: ExpectedFinding { rule_id: "tm-a".to_string(), path: "src/lib.rs".to_string(), line: 3 },
            }),
        ),
        (
            "tm-b".to_string(),
            Some(RuleCases {
                failing: vec![file("b.rs", "b\nfailing\n3\n4\n5\n6\nfinding\n")],
                passing: vec![file("b.rs", "b passing\n")],
                expected: ExpectedFinding { rule_id: "tm-b".to_string(), path: "b.rs".to_string(), line: 7 },
            }),
        ),
    ];
    let found: Vec<_> = rules.into_iter().map(|rule| (rule.id, rule.embedded_cases)).collect();
    assert_eq!(found, expected);
}

#[test]
fn case_problems_come_back_sorted_with_the_others() {
    let text = "version = 1\ncolour = 1\n[[rule]]\nid = \"tm-a\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"f\"\n\
                failing = { files = [\"gone.rs\"], path = \"gone.rs\", line = 1 }\n";
    // `failing = { files = [` is 21 chars, so "gone.rs" starts at line 8, column 22; the [[rule]] header is 3:1.
    let expected = vec![
        problem(2, 1, "unknown key `colour` in the top level", "Remove it, or use one of: version, rule, retired."),
        problem(
            3,
            1,
            "baseline rule \"tm-a\" has no passing case",
            "Add passing = { files = [...] } to the rule, and put its files in crates/rules/baseline/test/1/cases/tm-a/\
             passing/, listed in the set's cases in crates/rules/src/rule_file/test_1.rs.",
        ),
        problem(
            8,
            22,
            "case file \"gone.rs\" of baseline rule \"tm-a\" is not carried by baseline set test@1",
            "Put it at crates/rules/baseline/test/1/cases/tm-a/failing/gone.rs and list it in the set's cases in \
             crates/rules/src/rule_file/test_1.rs, or correct files.",
        ),
    ];
    assert_eq!(problems(text), expected);
}

#[test]
fn a_syntax_error_is_one_problem_at_its_position() {
    let found = problems("version = = 1\n");
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!((found[0].line, found[0].col), (1, 11), "{found:?}");
    assert!(found[0].what.starts_with("TOML syntax error: "), "{found:?}");
    assert_eq!(found[0].fix, "Correct the TOML syntax at this position.");
}

/// Carries `tm-a`'s case files, a case file and a YAML file no rule names; its text also has an unknown key.
const SPARE: BaselineSet = BaselineSet {
    name: "test",
    version: 1,
    ready: true,
    text: "version = 1\ncolour = 1\n[[rule]]\nid = \"tm-a\"\nengine = \"ruff\"\ncheck = \"E1\"\nfix = \"f\"\n\
           failing = { files = [\"a.rs\"], path = \"a.rs\", line = 1 }\npassing = { files = [\"a.rs\"] }\n",
    yaml: &[("spare.yml", "rule: spare\n")],
    cases: &[
        ("cases/tm-a/failing/a.rs", "x\n"),
        ("cases/tm-a/failing/spare.rs", "x\n"),
        ("cases/tm-a/passing/a.rs", "x\n"),
    ],
};

#[test]
fn every_carried_file_no_rule_names_is_a_problem_at_1_1_sorted_first() {
    let expected = vec![
        problem(
            1,
            1,
            "baseline set test@1 carries case file \"cases/tm-a/failing/spare.rs\" that no rule names",
            "Remove it from the set's cases in crates/rules/src/rule_file/test_1.rs and from \
             crates/rules/baseline/test/1/, or name it in a rule.",
        ),
        problem(
            1,
            1,
            "baseline set test@1 carries YAML file \"spare.yml\" that no rule names",
            "Remove it from the set's yaml in crates/rules/src/rule_file/test_1.rs and from \
             crates/rules/baseline/test/1/, or name it in a rule.",
        ),
        problem(2, 1, "unknown key `colour` in the top level", "Remove it, or use one of: version, rule, retired."),
    ];
    assert_eq!(load_set(&SPARE).expect_err("the set has problems"), expected);
}
