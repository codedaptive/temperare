//! ZUT for md_rule_of.rs: an id or alias in any case finds its rule; a tag, an unlisted id and a near miss find none.

use super::md_rule_of;

#[test]
fn md_rule_of_cases() {
    let rows: [(&str, Option<&str>); 9] = [
        ("MD013", Some("MD013")),
        ("md013", Some("MD013")),
        ("line-length", Some("MD013")),
        ("LINE-LENGTH", Some("MD013")),
        ("single-h1", Some("MD025")),
        ("headings", None),
        ("line_length", None),
        ("MD999", None),
        ("MD02", None),
    ];
    for (check, expected) in rows {
        assert_eq!(md_rule_of(check).map(|rule| rule.names[0]), expected, "{check}");
    }
}
