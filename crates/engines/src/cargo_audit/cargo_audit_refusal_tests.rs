//! Unit tests for cargo_audit_refusal.rs: the kinds and ids kept, `yanked` and other checks refused, texts in full.

use super::cargo_audit_refusal;

#[test]
fn cargo_audit_refusal_cases() {
    let other = |check: &str| {
        Some(format!("cargo-audit check `{check}` is not an advisory id (RUSTSEC-YYYY-NNNN) or a kind: all, vulnerability, unmaintained, unsound or notice"))
    };
    let yanked = Some("cargo-audit check `yanked` needs the crates.io index, which cctao never fetches; name an advisory id or a kind: all, vulnerability, unmaintained, unsound or notice".to_string());
    let rows: [(&str, &str, Option<String>); 13] = [
        ("all", "all", None),
        ("vulnerability", "vulnerability", None),
        ("unmaintained", "unmaintained", None),
        ("unsound", "unsound", None),
        ("notice", "notice", None),
        ("an advisory id", "RUSTSEC-2021-0003", None),
        ("yanked", "yanked", yanked),
        ("empty", "", other("")),
        ("a kind in capitals", "Unsound", other("Unsound")),
        ("an id with three-digit number", "RUSTSEC-2021-003", other("RUSTSEC-2021-003")),
        ("an id with letters", "RUSTSEC-2021-00a3", other("RUSTSEC-2021-00a3")),
        ("an id with a tail", "RUSTSEC-2021-0003-x", other("RUSTSEC-2021-0003-x")),
        ("a lower-case id", "rustsec-2021-0003", other("rustsec-2021-0003")),
    ];
    for (label, check, expected) in rows {
        assert_eq!(cargo_audit_refusal(check), expected, "{label}");
    }
}
