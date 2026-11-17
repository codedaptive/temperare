use std::ffi::OsString;
use std::path::{Path, PathBuf};

use super::toplevel::{toplevel_answer, toplevel_args};
use crate::git::GitError;
use crate::{Cmd, Output};

fn args(list: &[&str]) -> Vec<OsString> {
    list.iter().map(OsString::from).collect()
}

fn cmd() -> Cmd {
    Cmd { args: args(&["-C", "/r/sub", "rev-parse", "--show-toplevel"]), ..Cmd::new("git") }
}

fn out(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

#[test]
fn args_are_rev_parse_show_toplevel() {
    assert_eq!(toplevel_args(), args(&["rev-parse", "--show-toplevel"]));
}

#[test]
fn success_is_the_path_without_its_newline() {
    let got = toplevel_answer(&cmd(), out(0, b"/r\n", b""), Path::new("/r/sub")).unwrap();
    assert_eq!(got, PathBuf::from("/r"));
}

#[test]
fn only_one_trailing_newline_is_stripped() {
    let got = toplevel_answer(&cmd(), out(0, b"/r/odd\n\n", b""), Path::new("/r")).unwrap();
    assert_eq!(got, PathBuf::from("/r/odd\n"));
}

#[test]
fn not_a_git_repository_is_its_own_error_naming_the_repo_given() {
    let stderr = b"fatal: not a git repository (or any of the parent directories): .git\n";
    match toplevel_answer(&cmd(), out(128, b"", stderr), Path::new("/plain")) {
        Err(GitError::NotARepository { repo }) => assert_eq!(repo, PathBuf::from("/plain")),
        other => panic!("expected NotARepository, got {other:?}"),
    }
}

#[test]
fn a_missing_directory_is_exit() {
    let stderr = b"fatal: cannot change to '/gone': No such file or directory\n";
    match toplevel_answer(&cmd(), out(128, b"", stderr), Path::new("/gone")) {
        Err(GitError::Exit { args: a, code, stderr: e }) => {
            assert_eq!(a, args(&["-C", "/r/sub", "rev-parse", "--show-toplevel"]));
            assert_eq!(code, Some(128));
            assert_eq!(e, b"fatal: cannot change to '/gone': No such file or directory\n".to_vec());
        }
        other => panic!("expected Exit, got {other:?}"),
    }
}

#[test]
fn not_a_repository_text_under_another_code_is_exit() {
    match toplevel_answer(&cmd(), out(1, b"", b"not a git repository"), Path::new("/r")) {
        Err(GitError::Exit { code, .. }) => assert_eq!(code, Some(1)),
        other => panic!("expected Exit, got {other:?}"),
    }
}

#[test]
fn empty_stdout_is_bad_output() {
    for stdout in [&b""[..], b"\n"] {
        match toplevel_answer(&cmd(), out(0, stdout, b""), Path::new("/r")) {
            Err(GitError::BadOutput { args: a, stdout: s }) => {
                assert_eq!(a, args(&["-C", "/r/sub", "rev-parse", "--show-toplevel"]));
                assert_eq!(s, stdout.to_vec());
            }
            other => panic!("expected BadOutput, got {other:?}"),
        }
    }
}
