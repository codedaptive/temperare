//! Unit tests for shellcheck_version.rs.
//!
//! version.txt: stdout of `shellcheck --version` (shellcheck 0.11.0), exit 0.

use super::shellcheck_version;
use procio::Output;

fn out(code: i32, stdout: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: Vec::new() }
}

#[test]
fn the_version_line_of_a_real_probe_is_read() {
    assert_eq!(shellcheck_version(&out(0, include_bytes!("fixtures/version.txt"))), Some("0.11.0".to_string()));
}

#[test]
fn a_probe_that_did_not_exit_0_gives_none() {
    assert_eq!(shellcheck_version(&out(1, include_bytes!("fixtures/version.txt"))), None);
}

#[test]
fn stdout_with_no_version_line_gives_none() {
    assert_eq!(shellcheck_version(&out(0, b"ShellCheck - shell script analysis tool\nlicense: GPLv3\n")), None);
}

#[test]
fn an_empty_version_gives_none() {
    assert_eq!(shellcheck_version(&out(0, b"version:   \n")), None);
}
