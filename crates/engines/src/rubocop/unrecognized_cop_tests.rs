//! Unit test for unrecognized_cop.rs: the unknown-cop line of real rubocop stderr (check_unknown_stderr.txt, from
//! `--only=Style/NoSuchCop`; see fixtures/fixtures.txt), found when it names a part of the check and only then. The
//! `trimmed` row's stderr is hand-built, to show the line is trimmed.

use super::unrecognized_cop;
use procio::Output;

const STDERR: &[u8] = include_bytes!("fixtures/check_unknown_stderr.txt");

#[test]
fn unrecognized_cop_finds_the_line_naming_a_cop_of_the_check() {
    let cases: [(&str, &[u8], &str, Option<&str>); 6] = [
        ("the check", STDERR, "Style/NoSuchCop", Some("Unrecognized cop or department: Style/NoSuchCop.")),
        ("a part of a list", STDERR, "Lint/UselessAssignment,Style/NoSuchCop", Some("Unrecognized cop or department: Style/NoSuchCop.")),
        ("a cop the line's name only starts with", STDERR, "Style/NoSuch", None),
        ("another cop", STDERR, "Style/FileTouch", None),
        ("trimmed", b"  Unrecognized cop or department: Lint/Gone.  \n", "Lint/Gone", Some("Unrecognized cop or department: Lint/Gone.")),
        ("empty stderr", b"", "Style/NoSuchCop", None),
    ];
    let mut failed = Vec::new();
    for (name, stderr, check, want) in cases {
        let result = std::panic::catch_unwind(|| {
            let out = Output { code: Some(2), stdout: include_bytes!("fixtures/check_unknown.json").to_vec(), stderr: stderr.to_vec() };
            assert_eq!(unrecognized_cop(&out, check).as_deref(), want, "{name}");
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
