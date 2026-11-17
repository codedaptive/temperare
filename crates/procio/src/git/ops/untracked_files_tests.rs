use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};

use super::untracked_files::{untracked_files_answer, untracked_files_args};
use crate::git::{git_cmd, GitError};
use crate::{Cmd, Output};

fn args(list: &[&str]) -> Vec<OsString> {
    list.iter().map(OsString::from).collect()
}

fn cmd() -> Cmd {
    Cmd {
        args: args(&["-C", "/r", "ls-files", "--others", "--exclude-standard", "-z", "--full-name", "--", ":/"]),
        ..Cmd::new("git")
    }
}

fn out(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

#[test]
fn args_end_with_double_dash_then_the_top_pathspec() {
    assert_eq!(
        untracked_files_args(),
        args(&["ls-files", "--others", "--exclude-standard", "-z", "--full-name", "--", ":/"])
    );
}

#[test]
fn the_command_runs_ls_files_in_repo() {
    assert_eq!(
        git_cmd(Path::new("/r"), untracked_files_args()).args,
        args(&["-C", "/r", "ls-files", "--others", "--exclude-standard", "-z", "--full-name", "--", ":/"])
    );
}

#[test]
fn a_nul_list_is_the_paths_in_gits_order() {
    let got = untracked_files_answer(&cmd(), out(0, b"z.txt\0a.txt\0dir/c d.txt\0", b"")).unwrap();
    assert_eq!(got, vec![PathBuf::from("z.txt"), PathBuf::from("a.txt"), PathBuf::from("dir/c d.txt")]);
}

#[test]
fn empty_output_is_no_paths() {
    assert_eq!(untracked_files_answer(&cmd(), out(0, b"", b"")).unwrap(), Vec::<PathBuf>::new());
}

#[test]
fn non_utf8_path_bytes_are_kept() {
    let got = untracked_files_answer(&cmd(), out(0, b"bin\xffary\0", b"")).unwrap();
    assert_eq!(got, vec![PathBuf::from(OsString::from_vec(b"bin\xffary".to_vec()))]);
}

#[test]
fn exit_128_is_exit_with_args_code_and_stderr() {
    match untracked_files_answer(&cmd(), out(128, b"", b"fatal: not a git repository\n")) {
        Err(GitError::Exit { args: a, code, stderr }) => {
            assert_eq!(
                a,
                args(&["-C", "/r", "ls-files", "--others", "--exclude-standard", "-z", "--full-name", "--", ":/"])
            );
            assert_eq!(code, Some(128));
            assert_eq!(stderr, b"fatal: not a git repository\n".to_vec());
        }
        other => panic!("expected Exit, got {other:?}"),
    }
}

#[test]
fn exit_1_is_not_an_answer() {
    match untracked_files_answer(&cmd(), out(1, b"a.txt\0", b"")) {
        Err(GitError::Exit { code, .. }) => assert_eq!(code, Some(1)),
        other => panic!("expected Exit, got {other:?}"),
    }
}
