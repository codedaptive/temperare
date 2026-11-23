//! ZUT for markdownlint_version.rs: the version read from `--help` output. The first row is the line the installed
//! markdownlint-cli2 printed with exit 2 (fixtures/fixtures.txt); the others are built by hand from the contract.

use super::markdownlint_version;
use procio::Output;

#[test]
fn markdownlint_version_reads_the_banner_whatever_the_exit() {
    let help = "markdownlint-cli2 v0.24.2 (markdownlint v0.42.2)\nhttps://github.com/DavidAnson/markdownlint-cli2\n";
    let rows: [(&str, i32, &str, Option<&str>); 7] = [
        ("--help, exit 2", 2, help, Some("0.24.2")),
        ("exit 0", 0, help, Some("0.24.2")),
        ("blank line first", 2, "\n  markdownlint-cli2 v1.0.0 (markdownlint v1.0.0)\n", Some("1.0.0")),
        ("another program", 2, "markdownlint v0.42.2\n", None),
        ("no digit after v", 2, "markdownlint-cli2 vnext\n", None),
        ("banner not first", 0, "Usage\nmarkdownlint-cli2 v0.24.2\n", None),
        ("empty", 2, "", None),
    ];
    for (label, code, stdout, expected) in rows {
        let out = Output { code: Some(code), stdout: stdout.as_bytes().to_vec(), stderr: Vec::new() };
        assert_eq!(markdownlint_version(&out).as_deref(), expected, "{label}");
    }
}
