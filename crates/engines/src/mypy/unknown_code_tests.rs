//! Unit test for unknown_code.rs: mypy 2.4.1's invalid-code line picked out of its stderr when its list holds the rule's
//! check, and left alone otherwise. The stderr texts are mypy's, as printed for `--enable-error-code=nonsense-code`
//! and for that plus `--enable-error-code=other-bad`.

use super::unknown_code;
use procio::Output;

#[test]
fn unknown_code_cases() {
    let one = "mypy: error: Invalid error code(s): nonsense-code\nFound 1 error in 1 file (errors prevented further checking)\n";
    let two = "mypy: error: Invalid error code(s): nonsense-code, other-bad\n";
    let rows: [(&str, &str, &str, Option<&str>); 6] = [
        ("the one code", one, "nonsense-code", Some("mypy: error: Invalid error code(s): nonsense-code")),
        ("the second of two, trimmed", two, "other-bad", Some("mypy: error: Invalid error code(s): nonsense-code, other-bad")),
        ("another code", one, "arg-type", None),
        ("a code the listed one starts with", one, "nonsense", None),
        ("stderr without the words", "a.py: error: something else\n", "nonsense-code", None),
        ("empty stderr", "", "nonsense-code", None),
    ];
    for (label, stderr, check, expected) in rows {
        let out = Output { code: Some(2), stdout: Vec::new(), stderr: stderr.as_bytes().to_vec() };
        assert_eq!(unknown_code(&out, check).as_deref(), expected, "{label}");
    }
}
