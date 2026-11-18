//! ZUT for swiftlint_cmd.rs: the program, arguments and working directory of a swiftlint run.

use super::swiftlint_cmd;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

fn args(list: &[&str]) -> Vec<OsString> {
    list.iter().map(OsString::from).collect()
}

#[test]
fn without_a_config_it_lints_one_rule_as_json_in_the_folder() {
    let cmd = swiftlint_cmd("force_cast", None, Path::new("/w/app"));
    assert_eq!(cmd.program, OsString::from("swiftlint"));
    assert_eq!(cmd.args, args(&["lint", "--reporter", "json", "--quiet", "--only-rule=force_cast"]));
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/app")));
    assert!(cmd.env.is_empty() && cmd.env_remove.is_empty() && cmd.stdin.is_empty() && cmd.timeout.is_none());
}

#[test]
fn a_config_is_passed_last_as_written() {
    let cmd = swiftlint_cmd("force_cast", Some("lint/.swiftlint.yml"), Path::new("/w/app"));
    assert_eq!(
        cmd.args,
        args(&["lint", "--reporter", "json", "--quiet", "--only-rule=force_cast", "--config=lint/.swiftlint.yml"])
    );
}

#[test]
fn option_like_check_and_config_are_single_value_arguments() {
    let cmd = swiftlint_cmd("--help", Some("--isolated"), Path::new("/w/app"));
    assert_eq!(cmd.args, args(&["lint", "--reporter", "json", "--quiet", "--only-rule=--help", "--config=--isolated"]));
}
