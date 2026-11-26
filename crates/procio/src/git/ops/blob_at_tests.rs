use std::ffi::OsString;
#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;
use std::path::Path;

use super::blob_at::{blob_at_answer, blob_at_args};
use crate::git::GitError;
use crate::{Cmd, Output};

fn args(list: &[&str]) -> Vec<OsString> {
    list.iter().map(OsString::from).collect()
}

fn cmd() -> Cmd {
    Cmd { args: args(&["-C", "/r", "show", "--end-of-options", "HEAD:f.txt"]), ..Cmd::new("git") }
}

fn out(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

#[test]
fn args_are_one_rev_colon_path_object_after_end_of_options() {
    assert_eq!(blob_at_args("HEAD", Path::new("sub/g d.txt")), args(&["show", "--end-of-options", "HEAD:sub/g d.txt"]));
}

// Unix only: it builds a non-UTF-8 path from raw bytes, which only unix's OsString allows.
#[cfg(unix)]
#[test]
fn non_utf8_path_bytes_are_kept_in_the_object() {
    let path = OsString::from_vec(b"bin\xffary".to_vec());
    let want = vec!["show".into(), "--end-of-options".into(), OsString::from_vec(b"main:bin\xffary".to_vec())];
    assert_eq!(blob_at_args("main", Path::new(&path)), want);
}

#[test]
fn exit_0_is_the_bytes_untouched() {
    let got = blob_at_answer(&cmd(), out(0, b"bin\xffary\nlast line", b"")).unwrap();
    assert_eq!(got, Some(b"bin\xffary\nlast line".to_vec()));
}

#[test]
fn exit_0_keeps_a_trailing_newline() {
    assert_eq!(blob_at_answer(&cmd(), out(0, b"x\n", b"")).unwrap(), Some(b"x\n".to_vec()));
}

#[test]
fn a_path_missing_at_the_rev_is_none() {
    let stderr = b"fatal: path 'absent.txt' does not exist in 'HEAD'\n";
    assert_eq!(blob_at_answer(&cmd(), out(128, b"", stderr)).unwrap(), None);
}

#[test]
fn a_path_on_disk_but_not_at_the_rev_is_none() {
    let stderr = b"fatal: path 'new.txt' exists on disk, but not in 'HEAD'\n";
    assert_eq!(blob_at_answer(&cmd(), out(128, b"", stderr)).unwrap(), None);
}

#[test]
fn a_missing_rev_is_exit_with_args_code_and_stderr() {
    let stderr = b"fatal: invalid object name 'no-such-branch'.\n";
    match blob_at_answer(&cmd(), out(128, b"", stderr)) {
        Err(GitError::Exit { args: a, code, stderr: e }) => {
            assert_eq!(a, args(&["-C", "/r", "show", "--end-of-options", "HEAD:f.txt"]));
            assert_eq!(code, Some(128));
            assert_eq!(e, b"fatal: invalid object name 'no-such-branch'.\n".to_vec());
        }
        other => panic!("expected Exit, got {other:?}"),
    }
}

#[test]
fn the_missing_path_text_under_another_code_is_exit() {
    match blob_at_answer(&cmd(), out(1, b"", b"does not exist in")) {
        Err(GitError::Exit { code, .. }) => assert_eq!(code, Some(1)),
        other => panic!("expected Exit, got {other:?}"),
    }
}
