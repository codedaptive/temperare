//! Unit test for yamllint_refusal.rs: one rule id of `[a-z][a-z0-9-]*` passes, a name yamllint lacks included; anything else
//! is refused with an example id named. A backslash or quote never reaches the config key: the gate refuses it.

use super::yamllint_refusal;

#[test]
fn yamllint_refusal_cases() {
    let refused = |check: &str| {
        Some(format!("yamllint check `{check}` is not a yamllint rule id; a yamllint check is one rule id, such as `line-length`"))
    };
    let rows: [(&str, Option<String>); 13] = [
        ("truthy", None),
        ("line-length", None),
        ("yes", None),
        ("rule2", None),
        ("", refused("")),
        ("Truthy", refused("Truthy")),
        ("line_length", refused("line_length")),
        ("-x", refused("-x")),
        ("truthy,line-length", refused("truthy,line-length")),
        ("a: b", refused("a: b")),
        ("\"x\"", refused("\"x\"")),
        ("a\\b", refused("a\\b")),
        ("tru\"thy", refused("tru\"thy")),
    ];
    for (check, expected) in rows {
        assert_eq!(yamllint_refusal(check), expected, "{check:?}");
    }
}
