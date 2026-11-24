//! ZUT for vale_refusal.rs: `Style.Rule` with plain parts passes; anything else is refused with the form named.

use super::vale_refusal;

#[test]
fn vale_refusal_cases() {
    let refused =
        |check: &str| Some(format!("vale check `{check}` is not a Vale rule name (Style.Rule, each part ASCII letters, digits, _ or -)"));
    let rows: [(&str, Option<String>); 13] = [
        ("Vale.Spelling", None),
        ("write-good.Passive_2", None),
        ("Microsoft.Contractions", None),
        ("Spelling", refused("Spelling")),
        ("", refused("")),
        (".Spelling", refused(".Spelling")),
        ("Vale.", refused("Vale.")),
        ("Vale.Spelling.Extra", refused("Vale.Spelling.Extra")),
        ("Vale.Spel ling", refused("Vale.Spel ling")),
        ("Vale.\"x\"", refused("Vale.\"x\"")),
        ("Vale.Spelling\n", refused("Vale.Spelling\n")),
        ("Vale.Spélling", refused("Vale.Spélling")),
        ("Vale/Spelling", refused("Vale/Spelling")),
    ];
    for (check, expected) in rows {
        assert_eq!(vale_refusal(check), expected, "{check:?}");
    }
}
