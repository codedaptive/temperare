//! Unit tests for shellcheck_cmd.rs: the program, folder, removed environment and argv built for each config and file
//! case. Nothing runs.

use super::shellcheck_cmd;
use procio::Cmd;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

fn cmd(config: Option<&str>, files: &[&str]) -> Cmd {
    let files: Vec<PathBuf> = files.iter().map(PathBuf::from).collect();
    shellcheck_cmd(config, &files, Path::new("/no/such/folder"))
}

fn os(args: &[&str]) -> Vec<OsString> {
    args.iter().map(OsString::from).collect()
}

#[test]
fn the_program_is_shellcheck_run_in_the_folder_given() {
    let cmd = cmd(None, &["a.sh"]);
    assert_eq!(cmd.program, OsString::from("shellcheck"));
    assert_eq!(cmd.cwd, Some(PathBuf::from("/no/such/folder")));
}

#[test]
fn shellcheck_opts_is_removed_with_or_without_a_config() {
    assert_eq!(cmd(None, &["a.sh"]).env_remove, os(&["SHELLCHECK_OPTS"]));
    assert_eq!(cmd(Some("x.rc"), &["a.sh"]).env_remove, os(&["SHELLCHECK_OPTS"]));
}

#[test]
fn with_no_config_no_rc_file_is_read_and_the_files_follow_the_double_dash() {
    assert_eq!(cmd(None, &["a.sh", "dir/b.sh"]).args, os(&["-f", "json1", "--enable=all", "--norc", "--", "a.sh", "dir/b.sh"]));
}

#[test]
fn a_config_is_one_rcfile_argument_before_the_double_dash() {
    assert_eq!(cmd(Some("x.rc"), &["a.sh"]).args, os(&["-f", "json1", "--enable=all", "--rcfile=x.rc", "--", "a.sh"]));
}

#[test]
fn a_config_beginning_with_a_dash_stays_one_argument() {
    assert_eq!(cmd(Some("-weird"), &["a.sh"]).args, os(&["-f", "json1", "--enable=all", "--rcfile=-weird", "--", "a.sh"]));
}

#[test]
fn a_file_beginning_with_a_dash_comes_after_the_double_dash() {
    assert_eq!(cmd(None, &["-dash.sh"]).args, os(&["-f", "json1", "--enable=all", "--norc", "--", "-dash.sh"]));
}

#[test]
fn a_file_named_dash_is_passed_as_a_path_not_as_stdin() {
    assert_eq!(cmd(None, &["-", "-dash.sh"]).args, os(&["-f", "json1", "--enable=all", "--norc", "--", "./-", "-dash.sh"]));
}
