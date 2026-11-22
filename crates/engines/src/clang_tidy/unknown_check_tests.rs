//! ZUT for unknown_check.rs: clang-tidy's no-checks line found in its stderr (no_checks.txt is the real stderr of
//! `--checks=-*,nosuch-check`; fixtures/commands.txt), and nothing found elsewhere.

use super::unknown_check;
use procio::Output;

#[test]
fn unknown_check_cases() {
    let rows: [(&str, &[u8], Option<&str>); 4] = [
        ("the real stderr", include_bytes!("fixtures/no_checks.txt"), Some("Error: no checks enabled.")),
        ("the line indented", b"  Error: no checks enabled.  \n", Some("Error: no checks enabled.")),
        ("the words inside a longer line", b"note: Error: no checks enabled.\n", None),
        ("other stderr", b"Error while processing /w/a.c.\n", None),
    ];
    for (label, stderr, expected) in rows {
        let out = Output { code: Some(1), stdout: Vec::new(), stderr: stderr.to_vec() };
        assert_eq!(unknown_check(&out, "nosuch-check").as_deref(), expected, "{label}");
    }
}
