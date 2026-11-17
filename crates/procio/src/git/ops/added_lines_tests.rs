use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::PathBuf;

use super::added_lines::{added_lines_answer, added_lines_args};
use crate::git::GitError;
use crate::{Cmd, Output};

const FLAGS: [&str; 7] =
    ["diff", "-U0", "--no-color", "--no-renames", "--no-ext-diff", "--no-prefix", "--no-textconv"];

fn args(list: &[&str]) -> Vec<OsString> {
    list.iter().map(OsString::from).collect()
}

fn with_flags(rest: &[&str]) -> Vec<OsString> {
    args(&[&FLAGS[..], rest].concat())
}

fn cmd() -> Cmd {
    let mut a = args(&["-C", "/r"]);
    a.extend(with_flags(&["--end-of-options", "main", "--"]));
    Cmd { args: a, ..Cmd::new("git") }
}

fn out(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

#[test]
fn args_without_paths_end_with_double_dash() {
    assert_eq!(added_lines_args("main", &[]), with_flags(&["--end-of-options", "main", "--"]));
}

#[test]
fn args_put_the_paths_after_double_dash_in_order() {
    let paths = [PathBuf::from("b.txt"), PathBuf::from("dir/c d.txt"), PathBuf::from("-a.txt")];
    assert_eq!(
        added_lines_args("main", &paths),
        with_flags(&["--end-of-options", "main", "--", "b.txt", "dir/c d.txt", "-a.txt"])
    );
}

#[test]
fn a_diff_is_the_added_ranges_per_file() {
    let diff = b"diff --git f.txt f.txt\nindex 1..2 100644\n--- f.txt\n+++ f.txt\n\
@@ -2 +2 @@\n-two\n+TWO\n@@ -5 +5,2 @@\n-five\n+four-a\n+four-b\n\
diff --git g.txt g.txt\nindex 3..4 100644\n--- g.txt\n+++ g.txt\n@@ -2 +1,0 @@\n-b\n";
    let got = added_lines_answer(&cmd(), out(0, diff, b"")).unwrap();
    let want = BTreeMap::from([(PathBuf::from("f.txt"), vec![2..3, 5..7]), (PathBuf::from("g.txt"), vec![])]);
    assert_eq!(got, want);
}

#[test]
fn empty_output_is_an_empty_map() {
    assert_eq!(added_lines_answer(&cmd(), out(0, b"", b"")).unwrap(), BTreeMap::new());
}

#[test]
fn exit_128_is_exit_with_args_code_and_stderr() {
    match added_lines_answer(&cmd(), out(128, b"", b"fatal: bad revision 'main'\n")) {
        Err(GitError::Exit { args: a, code, stderr }) => {
            let mut want = args(&["-C", "/r"]);
            want.extend(with_flags(&["--end-of-options", "main", "--"]));
            assert_eq!(a, want);
            assert_eq!(code, Some(128));
            assert_eq!(stderr, b"fatal: bad revision 'main'\n".to_vec());
        }
        other => panic!("expected Exit, got {other:?}"),
    }
}

#[test]
fn exit_1_is_not_an_answer() {
    match added_lines_answer(&cmd(), out(1, b"", b"")) {
        Err(GitError::Exit { code, .. }) => assert_eq!(code, Some(1)),
        other => panic!("expected Exit, got {other:?}"),
    }
}

#[test]
fn an_unreadable_hunk_header_is_bad_output() {
    let diff = b"diff --git f.txt f.txt\n--- f.txt\n+++ f.txt\n@@ -1 +x @@\n+y\n";
    match added_lines_answer(&cmd(), out(0, diff, b"")) {
        Err(GitError::BadOutput { args: a, stdout }) => {
            let mut want = args(&["-C", "/r"]);
            want.extend(with_flags(&["--end-of-options", "main", "--"]));
            assert_eq!(a, want);
            assert_eq!(stdout, diff.to_vec());
        }
        other => panic!("expected BadOutput, got {other:?}"),
    }
}
