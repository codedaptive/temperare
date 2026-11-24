//! ZUT for vale_unknown.rs: the unknown-rule error object picked out of Vale 3.28.1's real stderr
//! (fixtures/unknown.txt; see fixtures/fixtures.txt) when it names the check, and None otherwise.

use super::vale_unknown;
use procio::Output;

#[test]
fn vale_unknown_cases() {
    let unknown: &[u8] = include_bytes!("fixtures/unknown.txt");
    let missing_style: &[u8] = include_bytes!("fixtures/missing_style.txt");
    let findings: &[u8] = include_bytes!("fixtures/findings.json");
    let said = "invalid operation: int(unknown) (1:86)\n | filter(Rules, {.Name == \"Vale.Speling\" || (none(Rules, {.Name == \"Vale.Speling\"}) && int(\"unknown\") == 0)})\n | .....................................................................................^";
    let rows: [(&str, &[u8], &[u8], &str, Option<&str>); 6] = [
        ("unknown rule", b"", unknown, "Vale.Speling", Some(said)),
        ("names another rule", b"", unknown, "Vale.Spelling", None),
        ("check only starts the name", b"", unknown, "Vale.Spel", None),
        ("another error", b"", missing_style, "Gone.Rule", None),
        ("findings, no error", findings, b"", "Vale.Spelling", None),
        ("the object on stdout", unknown, b"", "Vale.Speling", None),
    ];
    for (label, stdout, stderr, check, expected) in rows {
        let out = Output { code: Some(2), stdout: stdout.to_vec(), stderr: stderr.to_vec() };
        assert_eq!(vale_unknown(&out, check).as_deref(), expected, "{label}");
    }
}
