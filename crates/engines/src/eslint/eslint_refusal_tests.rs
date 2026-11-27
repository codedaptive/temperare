//! ZUT for eslint_refusal.rs: which checks are refused, the rows written out from the contract.

use super::eslint_refusal;

#[test]
fn eslint_refusal_refuses_what_is_not_a_rule_id() {
    let why = |check: &str| {
        Some(format!("eslint check `{check}` is not an ESLint rule id; cctao passes it to `--rule` and matches findings by it"))
    };
    let rows = [
        ("no-var", None),
        ("@typescript-eslint/no-explicit-any", None),
        ("react/jsx-key", None),
        ("import/no-unresolved", None),
        ("", why("")),
        ("no var", why("no var")),
        ("a,b", why("a,b")),
        ("x\"y", why("x\"y")),
        ("a:b", why("a:b")),
        ("a@b", why("a@b")),
        ("@", why("@")),
    ];
    let failed: Vec<&str> = rows.iter().filter(|(check, want)| eslint_refusal(check) != *want).map(|(check, _)| *check).collect();
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
