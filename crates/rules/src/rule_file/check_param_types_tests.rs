//! Unit tests for check_param_types.rs.

use super::check_param_types::check_param_types;
use super::param_type::ParamType;
use crate::Problem;
use toml::de::DeTable;

#[test]
fn a_typed_param_of_the_wrong_type_is_its_problem() {
    let list_fix = "Write `p` as a list of strings, e.g. `p = [\"...\"]`.";
    let problem = |col: usize, what: &str, fix: &str| Problem { line: 1, col, what: what.to_string(), fix: fix.to_string() };
    // `p = ` is 4 chars: the value starts at column 5.
    let rows: Vec<(&str, Option<ParamType>, Vec<Problem>)> = vec![
        ("p = [\"a\", \"b\"]\n", Some(ParamType::Strings), vec![]),
        ("p = \"a\"\n", Some(ParamType::String), vec![]),
        (
            "p = \"a\"\n",
            Some(ParamType::Strings),
            vec![problem(5, "param `p` of baseline rule \"tm-r\" must be a list of strings, not string", list_fix)],
        ),
        // `p = ["a", ` is 10 chars: 3 starts at column 11.
        (
            "p = [\"a\", 3]\n",
            Some(ParamType::Strings),
            vec![problem(11, "param `p` of baseline rule \"tm-r\" must be a list of strings; item 2 is integer", list_fix)],
        ),
        (
            "p = [\"a\"]\n",
            Some(ParamType::String),
            vec![problem(
                5,
                "param `p` of baseline rule \"tm-r\" must be a string, not array",
                "Write `p` as a string, e.g. `p = \"...\"`.",
            )],
        ),
        ("p = 3\n", None, vec![]),
        ("q = 3\n", Some(ParamType::Strings), vec![]),
    ];
    for (text, ty, problems) in rows {
        let doc = DeTable::parse(text).unwrap();
        let mut out = Vec::new();
        check_param_types(text, "tm-r", &[("p".to_string(), ty)], doc.get_ref(), &mut out);
        assert_eq!(out, problems, "{text} as {ty:?}");
    }
}
