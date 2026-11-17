//! ZUT for clippy_cmd.rs: the command, its arguments and where it runs.

use super::clippy_cmd;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[test]
fn runs_cargo_clippy_json_with_the_lint_warned_in_the_cargo_root() {
    let cmd = clippy_cmd("clippy::bool_comparison", Path::new("/w/crate"));
    assert_eq!(cmd.program, OsString::from("cargo"));
    let want: Vec<OsString> = ["clippy", "--message-format=json", "--quiet", "--", "-W", "clippy::bool_comparison"]
        .map(OsString::from)
        .into();
    assert_eq!(cmd.args, want);
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/crate")));
    assert!(cmd.env.is_empty());
    assert!(cmd.env_remove.is_empty());
    assert!(cmd.stdin.is_empty());
    assert_eq!(cmd.timeout, None);
}

#[test]
fn the_check_is_the_last_argument_as_given() {
    let cmd = clippy_cmd("unused_variables", Path::new("/w/crate"));
    assert_eq!(cmd.args.last(), Some(&OsString::from("unused_variables")));
}
