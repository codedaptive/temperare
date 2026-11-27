//! Unit test for unknown_rule.rs: sqlfluff 4.5.0's unknown-reference line picked out of its stderr when its list holds the
//! rule's check, and left alone otherwise. fixtures/unknown_rule.txt is its stderr for `--rules=NOPE99`
//! (fixtures/fixtures.txt).

use super::unknown_rule;
use procio::Output;

#[test]
fn unknown_rule_cases() {
    let saved = include_str!("fixtures/unknown_rule.txt");
    let said = "WARNING    Tried to allowlist unknown rule references: ['NOPE99']";
    let two = "WARNING    Tried to allowlist unknown rule references: ['NOPE99', 'lt01']\n";
    let rows: [(&str, &str, &str, Option<&str>); 7] = [
        ("the saved line", saved, "NOPE99", Some(said)),
        ("the second of two", two, "lt01", Some("WARNING    Tried to allowlist unknown rule references: ['NOPE99', 'lt01']")),
        ("another reference", saved, "LT01", None),
        ("a reference the listed one starts with", saved, "NOPE", None),
        ("a case that differs", saved, "nope99", None),
        ("stderr without the words", "WARNING    Length of file 'big.sql' is 25500 bytes\n", "NOPE99", None),
        ("empty stderr", "", "NOPE99", None),
    ];
    for (label, stderr, check, expected) in rows {
        let out = Output { code: Some(0), stdout: Vec::new(), stderr: stderr.as_bytes().to_vec() };
        assert_eq!(unknown_rule(&out, check).as_deref(), expected, "{label}");
    }
}
