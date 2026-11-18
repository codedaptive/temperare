//! ZUT for clippy_cmd.rs: the command, its full arguments with and without allows, and where it runs.

use super::clippy_cmd;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

fn os(args: &[&str]) -> Vec<OsString> {
    args.iter().map(OsString::from).collect()
}

#[test]
fn runs_cargo_clippy_json_over_all_targets_with_the_lint_warned_in_the_cargo_root() {
    let cmd = clippy_cmd("clippy::bool_comparison", &[], Path::new("/w/crate"));
    assert_eq!(cmd.program, OsString::from("cargo"));
    let want = os(&["clippy", "--all-targets", "--message-format=json", "--quiet", "--", "-W", "clippy::bool_comparison"]);
    assert_eq!(cmd.args, want);
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/crate")));
    assert!(cmd.env.is_empty());
    assert!(cmd.env_remove.is_empty());
    assert!(cmd.stdin.is_empty());
    assert_eq!(cmd.timeout, None);
}

#[test]
fn each_allow_follows_the_check_as_dash_a_in_order() {
    let allows = ["clippy::result_large_err".to_string(), "clippy::module_inception".to_string()];
    let cmd = clippy_cmd("warnings", &allows, Path::new("/w/crate"));
    let want = os(&[
        "clippy",
        "--all-targets",
        "--message-format=json",
        "--quiet",
        "--",
        "-W",
        "warnings",
        "-A",
        "clippy::result_large_err",
        "-A",
        "clippy::module_inception",
    ]);
    assert_eq!(cmd.args, want);
}

#[test]
fn the_check_is_the_last_argument_as_given_when_nothing_is_allowed() {
    let cmd = clippy_cmd("unused_variables", &[], Path::new("/w/crate"));
    assert_eq!(cmd.args.last(), Some(&OsString::from("unused_variables")));
}
