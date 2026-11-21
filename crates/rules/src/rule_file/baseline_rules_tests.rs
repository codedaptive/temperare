//! ZUT for baseline_rules.rs.

use super::baseline_rules::{baseline_rules, baseline_rules_in};
use super::baseline_set::BaselineSet;
use super::house_1::HOUSE_1;
use super::house_1_tests::expected_rows;
use super::load_set::load_set;
use crate::{CaseFile, ExpectedFinding, Problem, Rule, RuleCases};

/// A set not ready: an applicable ast-grep rule carrying its YAML, a ruff rule missing its required param whose
/// failing and passing cases give it different values, then another applicable ruff rule; each rule's case files
/// have their own path and text, so a swap shows.
const DRAFT: BaselineSet = BaselineSet {
    name: "draft",
    version: 1,
    ready: false,
    text: "version = 1\n\
           [[rule]]\nid = \"tm-d1\"\nengine = \"ast-grep\"\ncheck = \"ast-grep/tm-d1.yml\"\nfix = \"f\"\n\
           failing = { files = [\"a.rs\"], path = \"a.rs\", line = 2 }\npassing = { files = [\"a.rs\"] }\n\
           [[rule]]\nid = \"tm-d2\"\nengine = \"ruff\"\ncheck = \"d2\"\nfix = \"f\"\nrequired_params = [\"own_package\"]\n\
           failing = { files = [\"b.py\"], path = \"b.py\", line = 1, params = { own_package = [\"alpha\"] } }\n\
           passing = { files = [\"b.py\"], params = { own_package = [\"beta\"] } }\n\
           [[rule]]\nid = \"tm-d3\"\nengine = \"ruff\"\ncheck = \"d3\"\nfix = \"f\"\n\
           failing = { files = [\"c.py\"], path = \"c.py\", line = 3 }\npassing = { files = [\"c.py\"] }\n",
    yaml: &[("ast-grep/tm-d1.yml", "id: tm-d1\nlanguage: rust\n")],
    cases: &[
        ("cases/tm-d1/failing/a.rs", "fn a() {}\n// TODO\n"),
        ("cases/tm-d1/passing/a.rs", "fn a() {}\n"),
        ("cases/tm-d2/failing/b.py", "b = 1\n"),
        ("cases/tm-d2/passing/b.py", "b = 2\n"),
        ("cases/tm-d3/failing/c.py", "c = 1\nc = 2\nc = 3\n"),
        ("cases/tm-d3/passing/c.py", "c = 4\n"),
    ],
};

/// Each rule as (id, position, not_applicable reason, embedded YAML, embedded cases).
type Summary<'a> = (&'a str, usize, Option<&'a str>, Option<&'static str>, Option<RuleCases>);

fn summary(rules: &[Rule]) -> Vec<Summary<'_>> {
    rules
        .iter()
        .map(|rule| {
            let reason = rule.not_applicable.as_deref();
            (rule.id.as_str(), rule.position, reason, rule.embedded_yaml(), rule.cases().cloned())
        })
        .collect()
}

/// The cases of a one-file rule: `failing` and `passing` are (path, text), and the finding is at `line` of failing.
fn one_file_cases(id: &str, failing: (&str, &'static str), passing: (&str, &'static str), line: usize) -> RuleCases {
    let file = |(path, text): (&str, &'static str)| CaseFile { path: path.to_string(), text };
    let expected = ExpectedFinding { rule_id: id.to_string(), path: failing.0.to_string(), line };
    RuleCases { failing: vec![file(failing)], passing: vec![file(passing)], expected }
}

#[test]
fn a_set_not_ready_loads_its_rules_in_order() {
    let rules = baseline_rules_in("draft@1", &[DRAFT]).unwrap_or_else(|p| panic!("draft@1 should load: {p:?}"));
    let reason = "required params not given: own_package; give them in the params of an [[override]] for \"tm-d2\"";
    let d1 = one_file_cases("tm-d1", ("a.rs", "fn a() {}\n// TODO\n"), ("a.rs", "fn a() {}\n"), 2);
    let d2 = one_file_cases("tm-d2", ("b.py", "b = 1\n"), ("b.py", "b = 2\n"), 1);
    let d3 = one_file_cases("tm-d3", ("c.py", "c = 1\nc = 2\nc = 3\n"), ("c.py", "c = 4\n"), 3);
    assert_eq!(
        summary(&rules),
        vec![
            ("tm-d1", 0, None, Some("id: tm-d1\nlanguage: rust\n"), Some(d1)),
            ("tm-d2", 1, Some(reason), None, Some(d2)),
            ("tm-d3", 2, None, None, Some(d3)),
        ]
    );
    let own_package = |value: &str| -> toml::Table {
        [("own_package".to_string(), toml::Value::Array(vec![toml::Value::String(value.to_string())]))]
            .into_iter()
            .collect()
    };
    let params: Vec<_> =
        rules.iter().map(|rule| (rule.id.as_str(), rule.failing_params.clone(), rule.passing_params.clone())).collect();
    assert_eq!(
        params,
        vec![
            ("tm-d1", toml::Table::new(), toml::Table::new()),
            ("tm-d2", own_package("alpha"), own_package("beta")),
            ("tm-d3", toml::Table::new(), toml::Table::new()),
        ]
    );
}

#[test]
fn house_1_loads_every_rule_in_set_order() {
    let rules = baseline_rules("house@1").unwrap_or_else(|p| panic!("house@1 should load: {p:?}"));
    let carried = load_set(&HOUSE_1).unwrap_or_else(|p| panic!("house@1 has problems: {p:?}")).0;
    let found: Vec<_> =
        summary(&rules).into_iter().map(|(id, position, _, yaml, cases)| (id, position, yaml, cases)).collect();
    // Each rule's YAML is the file its check names when its engine is ast-grep, and its cases are the set's own.
    let expected: Vec<_> = expected_rows()
        .into_iter()
        .zip(&carried)
        .enumerate()
        .map(|(position, (row, set_rule))| {
            let yaml = HOUSE_1.yaml_for(row.2).filter(|_| row.1 == "ast-grep");
            (row.0, position, yaml, set_rule.cases().cloned())
        })
        .collect();
    assert_eq!(found, expected);
    // The five required-param rules' cases give their params, the same for failing and passing; every other rule's
    // cases give none.
    let params: Vec<_> =
        rules.iter().map(|rule| (rule.id.as_str(), rule.failing_params.clone(), rule.passing_params.clone())).collect();
    let given = |name: &str, value: &str| -> toml::Table {
        [(name.to_string(), toml::Value::Array(vec![toml::Value::String(value.to_string())]))].into_iter().collect()
    };
    let expected_params: Vec<_> = expected_rows()
        .into_iter()
        .map(|row| {
            let case = match row.0 {
                "tm-house-no-stored-bool-swift"
                | "tm-house-no-stored-bool-rust"
                | "tm-house-bitmap-int64-swift"
                | "tm-house-bitmap-int64-rust" => given("entity_types", "Drawer"),
                "tm-house-unit-test-fakes-own-kit-python" => given("own_package", "ledger"),
                _ => toml::Table::new(),
            };
            (row.0, case.clone(), case)
        })
        .collect();
    assert_eq!(params, expected_params);
    let not_applicable: Vec<&str> =
        rules.iter().filter(|rule| rule.not_applicable.is_some()).map(|rule| rule.id.as_str()).collect();
    assert_eq!(
        not_applicable,
        vec![
            "tm-house-no-stored-bool-swift",
            "tm-house-no-stored-bool-rust",
            "tm-house-bitmap-int64-swift",
            "tm-house-bitmap-int64-rust",
            "tm-house-unit-test-fakes-own-kit-python",
        ]
    );
}

#[test]
fn an_unknown_set_is_its_problem() {
    let expected = Problem {
        line: 2,
        col: 13,
        what: "unknown baseline set \"nosuch\"".to_string(),
        fix: "Use one of: house, standard.".to_string(),
    };
    assert_eq!(baseline_rules("nosuch@1").expect_err("nosuch@1 is no set"), vec![expected]);
}
