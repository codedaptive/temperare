//! ZUT for read_required_params.rs.

use super::param_type::ParamType;
use super::read_required_params::read_required_params;
use crate::Problem;
use toml::de::DeTable;

/// The `Problem` at line 1, column `col`.
fn at(col: usize, what: &str, fix: &str) -> Problem {
    Problem { line: 1, col, what: what.to_string(), fix: fix.to_string() }
}

#[test]
fn required_params_are_read_with_their_types_and_each_bad_one_is_its_problem() {
    let expected_fix = "Write `required_params` as an array of param names or { name, type } tables.";
    let untyped = |name: &str| (name.to_string(), None);
    let typed = |name: &str, ty| (name.to_string(), Some(ty));
    // `required_params = [` is 19 chars, so the first item starts at column 20.
    let rows: Vec<(&str, Vec<(String, Option<ParamType>)>, Vec<Problem>)> = vec![
        ("required_params = [\"a\", \"b\"]\n", vec![untyped("a"), untyped("b")], vec![]),
        (
            "required_params = [{ name = \"own_package\", type = \"strings\" }, { name = \"x\", type = \"string\" }]\n",
            vec![typed("own_package", ParamType::Strings), typed("x", ParamType::String)],
            vec![],
        ),
        // `{ name = "p", type = ` is 21 chars from column 20: "int" starts at column 41.
        (
            "required_params = [{ name = \"p\", type = \"int\" }]\n",
            vec![],
            vec![at(
                41,
                "unknown param type \"int\" for required param \"p\"",
                "Write type = \"string\" or type = \"strings\".",
            )],
        ),
        // `"a", ` is 5 chars from column 20: 3 starts at column 25.
        (
            "required_params = [\"a\", 3]\n",
            vec![untyped("a")],
            vec![at(
                25,
                "`required_params` must be an array of param names or { name, type } tables, not integer",
                expected_fix,
            )],
        ),
        // `required_params = ` is 18 chars: the value starts at column 19.
        (
            "required_params = \"a\"\n",
            vec![],
            vec![at(
                19,
                "`required_params` must be an array of param names or { name, type } tables, not string",
                expected_fix,
            )],
        ),
        ("x = 1\n", vec![], vec![]),
    ];
    for (text, params, problems) in rows {
        let doc = DeTable::parse(text).unwrap();
        let mut out = Vec::new();
        assert_eq!(read_required_params(text, doc.get_ref(), &mut out), params, "{text}");
        assert_eq!(out, problems, "{text}");
    }
}
