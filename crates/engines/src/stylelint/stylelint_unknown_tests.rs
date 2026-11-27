//! Unit test for stylelint_unknown.rs: stylelint's `Unknown rule` warning picked out of real stderr (fixtures/unknown.txt;
//! see fixtures/fixtures.txt) when its rule is the check, and None otherwise.

use super::stylelint_unknown;
use procio::Output;

#[test]
fn stylelint_unknown_finds_the_checks_unknown_rule_warning() {
    let unknown: &[u8] = include_bytes!("fixtures/unknown.txt");
    let rows: [(&str, &[u8], &str, Option<&str>); 5] = [
        ("the check is unknown", unknown, "color-no-invalid-hexx", Some("Unknown rule color-no-invalid-hexx. Did you mean color-no-invalid-hex?")),
        ("another rule is unknown", unknown, "color-no-invalid-hex", None),
        ("the check only starts the name", unknown, "color-no-invalid", None),
        ("findings, nothing unknown", include_bytes!("fixtures/findings.txt"), "color-no-invalid-hex", None),
        ("no report", include_bytes!("fixtures/config_error.txt"), "color-no-invalid-hex", None),
    ];
    let failed: Vec<&str> = rows
        .iter()
        .filter(|(_, stderr, check, want)| {
            let out = Output { code: Some(2), stdout: Vec::new(), stderr: stderr.to_vec() };
            stylelint_unknown(&out, check).as_deref() != *want
        })
        .map(|(name, ..)| *name)
        .collect();
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
