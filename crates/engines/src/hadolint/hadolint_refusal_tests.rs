//! Unit test for hadolint_refusal.rs: which checks are refused and the reason given, the cases written out from the
//! contract.

use super::hadolint_refusal;

#[test]
fn hadolint_refusal_cases() {
    let refused = |check: &str| {
        Some(format!(
            "hadolint check `{check}` is not one hadolint code such as `DL3006`, so cctao cannot tell which findings it selects"
        ))
    };
    let rows = [
        ("a DL code", "DL3006", None),
        ("one digit", "DL1", None),
        ("the parse failure's code", "DL1000", None),
        ("an SC code", "SC2086", None),
        ("a comma list", "DL3006,DL3008", refused("DL3006,DL3008")),
        ("ALL", "ALL", refused("ALL")),
        ("no digits", "DL", refused("DL")),
        ("a letter in the digits", "DLx", refused("DLx")),
        ("lower case", "dl3006", refused("dl3006")),
        ("another prefix", "XX1234", refused("XX1234")),
        ("a space", "DL3006 ", refused("DL3006 ")),
        ("empty", "", refused("")),
    ];
    for (label, check, expected) in rows {
        assert_eq!(hadolint_refusal(check), expected, "{label}");
    }
}
