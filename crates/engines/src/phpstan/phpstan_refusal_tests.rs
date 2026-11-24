//! ZUT for phpstan_refusal.rs: digits and `max` pass; anything else is refused with the level range named.

use super::phpstan_refusal;

#[test]
fn phpstan_refusal_cases() {
    let refused = |check: &str| Some(format!("phpstan check `{check}` is not a PHPStan level (0 to 10, or max)"));
    let rows: [(&str, Option<String>); 11] = [
        ("0", None),
        ("5", None),
        ("10", None),
        ("11", None),
        ("max", None),
        ("", refused("")),
        ("five", refused("five")),
        ("MAX", refused("MAX")),
        (" 5", refused(" 5")),
        ("-1", refused("-1")),
        ("5,6", refused("5,6")),
    ];
    for (check, expected) in rows {
        assert_eq!(phpstan_refusal(check), expected, "{check:?}");
    }
}
