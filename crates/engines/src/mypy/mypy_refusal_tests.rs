//! ZUT for mypy_refusal.rs: which checks are refused and the reason given, the cases written out from the contract.

use super::mypy_refusal;

#[test]
fn mypy_refusal_cases() {
    let refused = |check: &str| {
        Some(format!(
            "mypy check `{check}` is not one mypy error code (lowercase words joined by `-`, as `arg-type`), so tm cannot tell which findings it selects"
        ))
    };
    let rows: [(&str, &str, Option<String>); 14] = [
        ("two words", "arg-type", None),
        ("one word", "misc", None),
        ("three words", "no-untyped-def", None),
        ("import-not-found", "import-not-found", None),
        ("a digit in a word", "py3-only", None),
        ("empty", "", refused("")),
        ("capitalised", "Arg-type", refused("Arg-type")),
        ("a list", "arg-type,misc", refused("arg-type,misc")),
        ("a space", "arg type", refused("arg type")),
        ("a leading dash", "-misc", refused("-misc")),
        ("a trailing dash", "misc-", refused("misc-")),
        ("a double dash", "arg--type", refused("arg--type")),
        ("a leading digit", "3misc", refused("3misc")),
        ("an underscore", "arg_type", refused("arg_type")),
    ];
    for (label, check, expected) in rows {
        assert_eq!(mypy_refusal(check), expected, "{label}");
    }
}
