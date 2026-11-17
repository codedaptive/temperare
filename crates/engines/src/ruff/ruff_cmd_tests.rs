//! ZUT for ruff_cmd.rs: the program, arguments and working directory of a ruff run.

use super::ruff_cmd;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

fn args(list: &[&str]) -> Vec<OsString> {
    list.iter().map(OsString::from).collect()
}

#[test]
fn without_a_config_it_checks_one_rule_as_json_over_the_folder_with_no_cache() {
    let cmd = ruff_cmd("E711", None, Path::new("/w/py"));
    assert_eq!(cmd.program, OsString::from("ruff"));
    assert_eq!(cmd.args, args(&["check", "--no-cache", "--output-format", "json", "--select", "E711", "."]));
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/py")));
    assert!(cmd.env.is_empty() && cmd.env_remove.is_empty() && cmd.stdin.is_empty() && cmd.timeout.is_none());
}

#[test]
fn a_config_comes_before_the_folder_as_written() {
    let cmd = ruff_cmd("E711", Some("lint/ruff.toml"), Path::new("/w/py"));
    assert_eq!(
        cmd.args,
        args(&[
            "check",
            "--no-cache",
            "--output-format",
            "json",
            "--select",
            "E711",
            "--config",
            "lint/ruff.toml",
            "."
        ])
    );
}
