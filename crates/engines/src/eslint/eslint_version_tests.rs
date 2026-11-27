//! Unit test for eslint_version.rs: the version read from `eslint --version` output. `v10.15.0\n` is what the installed
//! eslint printed (fixtures/fixtures.txt); the other rows are hand-built from the contract.

use super::eslint_version;
use procio::Output;

#[test]
fn eslint_version_reads_the_first_line_without_its_v() {
    let rows: [(&str, i32, &str, Option<&str>); 6] = [
        ("eslint 10.15.0", 0, "v10.15.0\n", Some("10.15.0")),
        ("blank line first", 0, "\n  v9.0.1  \n", Some("9.0.1")),
        ("no v", 0, "10.15.0\n", Some("10.15.0")),
        ("non-zero exit", 1, "v10.15.0\n", None),
        ("not a version", 0, "version 10\n", None),
        ("empty", 0, "", None),
    ];
    let failed: Vec<&str> = rows
        .iter()
        .filter(|(_, code, stdout, want)| {
            let out = Output { code: Some(*code), stdout: stdout.as_bytes().to_vec(), stderr: Vec::new() };
            eslint_version(&out).as_deref() != *want
        })
        .map(|(name, ..)| *name)
        .collect();
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
