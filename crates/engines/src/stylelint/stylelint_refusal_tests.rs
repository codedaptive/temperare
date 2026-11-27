//! Unit test for stylelint_refusal.rs: which checks are refused, the rows written out from the contract.

use super::stylelint_refusal;

#[test]
fn stylelint_refusal_refuses_what_is_not_a_rule_name() {
    let why = |check: &str| {
        Some(format!("stylelint check `{check}` is not a stylelint rule name; cctao matches findings by it"))
    };
    let rows = [
        ("color-no-invalid-hex", None),
        ("scss/at-rule-no-unknown", None),
        ("@stylistic/indentation", None),
        ("plugin2/rule-3", None),
        ("", why("")),
        ("@", why("@")),
        ("CssSyntaxError", why("CssSyntaxError")),
        ("Color-named", why("Color-named")),
        ("color named", why("color named")),
        ("color_named", why("color_named")),
        ("a.b", why("a.b")),
        ("a,b", why("a,b")),
        ("a@b", why("a@b")),
    ];
    let failed: Vec<&str> =
        rows.iter().filter(|(check, want)| stylelint_refusal(check) != *want).map(|(check, _)| *check).collect();
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
