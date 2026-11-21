//! Unit tests for read_case_params.rs.

use super::param_type::ParamType;
use super::read_case_params::read_case_params;
use crate::Problem;
use toml::de::DeTable;

fn problem(col: usize, what: &str, fix: &str) -> Problem {
    Problem { line: 1, col, what: what.to_string(), fix: fix.to_string() }
}

#[test]
fn a_cases_params_must_be_a_table_with_no_dashed_allow_and_typed_params_of_their_type() {
    let list_fix = "Write `entity_types` as a list of strings, e.g. `entity_types = [\"...\"]`.";
    let dash_fix = "A check, config or allow value is never an engine option: remove the leading '-'.";
    // `params = ` is 9 chars, `params = { entity_types = ` 26 and `params = { allow = [` 20, so each faulty value
    // starts at column 10, 27 and 21.
    let rows = [
        ("", vec![]),
        ("params = { entity_types = [\"Drawer\"], allow = [\"x\"], depth = 2 }", vec![]),
        ("params = \"entity_types\"", vec![problem(10, "`params` must be a table, not string", "Write `params` as a table.")]),
        (
            "params = { entity_types = \"Drawer\" }",
            vec![problem(
                27,
                "param `entity_types` of baseline rule \"tm-a\" must be a list of strings, not string",
                list_fix,
            )],
        ),
        ("params = { allow = [\"-W\"] }", vec![problem(21, "params.allow \"-W\" begins with '-'", dash_fix)]),
    ];
    let types = [("entity_types".to_string(), Some(ParamType::Strings))];
    for (text, expected) in rows {
        let doc = DeTable::parse(text).expect("valid TOML");
        let mut out = Vec::new();
        read_case_params(text, doc.get_ref(), "tm-a", &types, &mut out);
        assert_eq!(out, expected, "{text}");
    }
}
