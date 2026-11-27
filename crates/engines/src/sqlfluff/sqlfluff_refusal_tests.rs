//! ZUT for sqlfluff_refusal.rs: which checks are refused and the reason given, the cases written out from the
//! contract.

use super::sqlfluff_refusal;

#[test]
fn sqlfluff_refusal_cases() {
    let refused = |check: &str| {
        Some(format!(
            "sqlfluff check `{check}` is not one sqlfluff rule code, name or group (as `LT01`, `layout.spacing` or `layout`), so cctao cannot pass it as one rule"
        ))
    };
    let rows: [(&str, &str, Option<String>); 11] = [
        ("a code", "LT01", None),
        ("a name", "layout.spacing", None),
        ("a group", "layout", None),
        ("an alias", "L039", None),
        ("an underscore", "capitalisation.keywords_x", None),
        ("empty", "", refused("")),
        ("a comma list", "LT01,LT02", refused("LT01,LT02")),
        ("a space", "LT01 LT02", refused("LT01 LT02")),
        ("a leading dot", ".spacing", refused(".spacing")),
        ("a dash", "layout-spacing", refused("layout-spacing")),
        ("a glob", "LT*", refused("LT*")),
    ];
    for (label, check, expected) in rows {
        assert_eq!(sqlfluff_refusal(check), expected, "{label}");
    }
}
