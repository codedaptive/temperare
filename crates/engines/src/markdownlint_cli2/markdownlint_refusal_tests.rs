//! ZUT for markdownlint_refusal.rs: a rule name's form (letters, digits, `-`) passes, listed or not; anything else is
//! refused.

use super::markdownlint_refusal;

#[test]
fn markdownlint_refusal_cases() {
    let refused = |check: &str| {
        Some(format!(
            "markdownlint check `{check}` is not a markdownlint rule name (letters, digits and `-`, as MD013 or line-length)"
        ))
    };
    let rows: [(&str, Option<String>); 9] = [
        ("MD013", None),
        ("md013", None),
        ("line-length", None),
        ("MD999", None),
        ("", refused("")),
        ("MD 013", refused("MD 013")),
        ("line_length", refused("line_length")),
        ("MD013,MD009", refused("MD013,MD009")),
        ("MD0é3", refused("MD0é3")),
    ];
    for (check, expected) in rows {
        assert_eq!(markdownlint_refusal(check), expected, "{check:?}");
    }
}
