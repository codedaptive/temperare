use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};

use super::tracked_files::{tracked_files_answer, tracked_files_args};
use crate::git::{git_cmd, GitError};
use crate::{Cmd, Output};

fn args(list: &[&str]) -> Vec<OsString> {
    list.iter().map(OsString::from).collect()
}

fn cmd() -> Cmd {
    Cmd {
        args: args(&["-C", "/r", "ls-files", "-z", "-t", "--cached", "--deleted", "--full-name", "--"]),
        ..Cmd::new("git")
    }
}

fn out(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

fn paths(list: &[&str]) -> Vec<PathBuf> {
    list.iter().map(PathBuf::from).collect()
}

#[test]
fn args_without_pathspecs_end_with_double_dash() {
    assert_eq!(
        tracked_files_args::<&str>(&[]),
        args(&["ls-files", "-z", "-t", "--cached", "--deleted", "--full-name", "--"])
    );
}

#[test]
fn args_put_pathspecs_after_double_dash_in_order() {
    assert_eq!(
        tracked_files_args(&["*.sh", "-n", "sub/x.txt"]),
        args(&["ls-files", "-z", "-t", "--cached", "--deleted", "--full-name", "--", "*.sh", "-n", "sub/x.txt"])
    );
}

#[test]
fn args_take_os_string_pathspecs() {
    assert_eq!(
        tracked_files_args(&[OsString::from_vec(b"bin\xffary".to_vec())]),
        vec![
            OsString::from("ls-files"),
            OsString::from("-z"),
            OsString::from("-t"),
            OsString::from("--cached"),
            OsString::from("--deleted"),
            OsString::from("--full-name"),
            OsString::from("--"),
            OsString::from_vec(b"bin\xffary".to_vec()),
        ]
    );
}

#[test]
fn the_command_runs_ls_files_in_repo() {
    assert_eq!(
        git_cmd(Path::new("/r"), tracked_files_args(&["*.sh"])).args,
        args(&["-C", "/r", "ls-files", "-z", "-t", "--cached", "--deleted", "--full-name", "--", "*.sh"])
    );
}

#[test]
fn cached_entries_are_the_paths_in_gits_order() {
    let got = tracked_files_answer(&cmd(), out(0, b"H z.txt\0H a.txt\0H dir/c d.txt\0", b"")).unwrap();
    assert_eq!(got, paths(&["z.txt", "a.txt", "dir/c d.txt"]));
}

#[test]
fn empty_output_is_no_paths() {
    assert_eq!(tracked_files_answer(&cmd(), out(0, b"", b"")).unwrap(), Vec::<PathBuf>::new());
}

#[test]
fn a_deleted_path_is_dropped() {
    let got = tracked_files_answer(&cmd(), out(0, b"H a\0H gone\0R gone\0H z\0", b"")).unwrap();
    assert_eq!(got, paths(&["a", "z"]));
}

#[test]
fn a_deleted_path_alone_is_no_paths() {
    let got = tracked_files_answer(&cmd(), out(0, b"H gone\0R gone\0", b"")).unwrap();
    assert_eq!(got, Vec::<PathBuf>::new());
}

#[test]
fn an_unmerged_path_is_listed_once() {
    let got = tracked_files_answer(&cmd(), out(0, b"M u\0M u\0M u\0", b"")).unwrap();
    assert_eq!(got, paths(&["u"]));
}

#[test]
fn a_skip_worktree_path_is_dropped() {
    let got = tracked_files_answer(&cmd(), out(0, b"S s\0H h\0", b"")).unwrap();
    assert_eq!(got, paths(&["h"]));
}

#[test]
fn an_entry_without_a_tag_and_space_is_dropped() {
    let got = tracked_files_answer(&cmd(), out(0, b"Hx\0H\0H ok\0", b"")).unwrap();
    assert_eq!(got, paths(&["ok"]));
}

#[test]
fn non_utf8_path_bytes_are_kept() {
    let got = tracked_files_answer(&cmd(), out(0, b"H bin\xffary\0", b"")).unwrap();
    assert_eq!(got, vec![PathBuf::from(OsString::from_vec(b"bin\xffary".to_vec()))]);
}

#[test]
fn exit_128_is_exit_with_args_code_and_stderr() {
    match tracked_files_answer(&cmd(), out(128, b"", b"fatal: not a git repository\n")) {
        Err(GitError::Exit { args: a, code, stderr }) => {
            assert_eq!(a, args(&["-C", "/r", "ls-files", "-z", "-t", "--cached", "--deleted", "--full-name", "--"]));
            assert_eq!(code, Some(128));
            assert_eq!(stderr, b"fatal: not a git repository\n".to_vec());
        }
        other => panic!("expected Exit, got {other:?}"),
    }
}

#[test]
fn exit_1_is_not_an_answer() {
    match tracked_files_answer(&cmd(), out(1, b"H a.txt\0", b"")) {
        Err(GitError::Exit { code, .. }) => assert_eq!(code, Some(1)),
        other => panic!("expected Exit, got {other:?}"),
    }
}
