use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;

use super::changed_files::{changed_files_answer, changed_files_args};
use crate::git::GitError;
use crate::{Cmd, Output};

fn args(list: &[&str]) -> Vec<OsString> {
    list.iter().map(OsString::from).collect()
}

fn cmd() -> Cmd {
    Cmd {
        args: args(&["-C", "/r", "diff", "--name-only", "-z", "--no-renames", "--end-of-options", "main", "--"]),
        ..Cmd::new("git")
    }
}

fn out(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

#[test]
fn args_put_the_base_after_end_of_options_and_end_with_double_dash() {
    assert_eq!(
        changed_files_args("main"),
        args(&["diff", "--name-only", "-z", "--no-renames", "--end-of-options", "main", "--"])
    );
}

#[test]
fn an_option_shaped_base_stays_after_end_of_options() {
    assert_eq!(
        changed_files_args("--output=x"),
        args(&["diff", "--name-only", "-z", "--no-renames", "--end-of-options", "--output=x", "--"])
    );
}

#[test]
fn a_nul_list_is_the_paths_in_gits_order() {
    let got = changed_files_answer(&cmd(), out(0, b"z.txt\0a.txt\0dir/c d.txt\0", b"")).unwrap();
    assert_eq!(got, vec![PathBuf::from("z.txt"), PathBuf::from("a.txt"), PathBuf::from("dir/c d.txt")]);
}

#[test]
fn empty_output_is_no_paths() {
    assert_eq!(changed_files_answer(&cmd(), out(0, b"", b"")).unwrap(), Vec::<PathBuf>::new());
}

#[test]
fn non_utf8_path_bytes_are_kept() {
    let got = changed_files_answer(&cmd(), out(0, b"bin\xffary\0", b"")).unwrap();
    assert_eq!(got, vec![PathBuf::from(OsString::from_vec(b"bin\xffary".to_vec()))]);
}

#[test]
fn exit_128_is_exit_with_args_code_and_stderr() {
    match changed_files_answer(&cmd(), out(128, b"", b"fatal: bad revision 'main'\n")) {
        Err(GitError::Exit { args: a, code, stderr }) => {
            assert_eq!(
                a,
                args(&["-C", "/r", "diff", "--name-only", "-z", "--no-renames", "--end-of-options", "main", "--"])
            );
            assert_eq!(code, Some(128));
            assert_eq!(stderr, b"fatal: bad revision 'main'\n".to_vec());
        }
        other => panic!("expected Exit, got {other:?}"),
    }
}

#[test]
fn exit_1_is_not_an_answer() {
    match changed_files_answer(&cmd(), out(1, b"a.txt\0", b"")) {
        Err(GitError::Exit { code, .. }) => assert_eq!(code, Some(1)),
        other => panic!("expected Exit, got {other:?}"),
    }
}
