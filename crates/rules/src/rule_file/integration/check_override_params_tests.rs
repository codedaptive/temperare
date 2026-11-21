//! ZIT for check_override_params.rs, through parse_with_sets.

use super::super::baseline_set::BaselineSet;
use super::super::rule_file_parse::parse_with_sets;
use crate::Problem;
use std::path::Path;

/// `pkg@1`'s one rule requires `own_package`, typed a list of strings.
const SETS: &[BaselineSet] = &[BaselineSet {
    name: "pkg",
    version: 1,
    ready: true,
    text: "version = 1\n[[rule]]\nid = \"tm-p1\"\nengine = \"ruff\"\ncheck = \"p1\"\nfix = \"f\"\n\
           required_params = [{ name = \"own_package\", type = \"strings\" }]\n\
           failing = { files = [\"a.py\"], path = \"a.py\", line = 1 }\npassing = { files = [\"a.py\"] }\n",
    yaml: &[],
    cases: &[("cases/tm-p1/failing/a.py", "x\n"), ("cases/tm-p1/passing/a.py", "x\n")],
}];

/// Each applicable rule as (id, params, not_applicable).
type Summary = (String, toml::Table, Option<String>);

#[test]
fn override_params_are_checked_against_the_types_the_set_declares() {
    let fix = "Write `own_package` as a list of strings, e.g. `own_package = [\"...\"]`.";
    let problem = |col: usize, what: &str| Problem { line: 5, col, what: what.to_string(), fix: fix.to_string() };
    let mut drawer = toml::Table::new();
    drawer.insert("own_package".to_string(), toml::Value::Array(vec![toml::Value::String("drawer".to_string())]));
    // Line 5 is the params line; `params = { own_package = ` is 25 chars, so the value starts at column 26.
    let rows: Vec<(&str, Result<Vec<Summary>, Vec<Problem>>)> = vec![
        ("[\"drawer\"]", Ok(vec![("tm-p1".to_string(), drawer, None)])),
        (
            "\"drawer\"",
            Err(vec![problem(26, "param `own_package` of baseline rule \"tm-p1\" must be a list of strings, not string")]),
        ),
        // `["drawer", ` is 11 chars from column 26: 3 starts at column 37.
        (
            "[\"drawer\", 3]",
            Err(vec![problem(
                37,
                "param `own_package` of baseline rule \"tm-p1\" must be a list of strings; item 2 is integer",
            )]),
        ),
    ];
    for (value, expected) in rows {
        let text = format!(
            "version = 1\nbaseline = [\"pkg@1\"]\n[[override]]\nrule = \"tm-p1\"\nparams = {{ own_package = {value} }}\n"
        );
        let found = parse_with_sets(&text, Path::new("."), SETS).map(|file| {
            file.rules.into_iter().map(|rule| (rule.id, rule.params, rule.not_applicable)).collect::<Vec<Summary>>()
        });
        assert_eq!(found, expected, "{value}");
    }
}
