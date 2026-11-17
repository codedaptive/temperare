use std::ffi::OsString;
use std::path::PathBuf;

use super::GitError;
use crate::ProcError;

#[test]
fn run_carries_the_proc_error() {
    let err = GitError::Run(ProcError::NotFound { program: "git".into() });
    let GitError::Run(ProcError::NotFound { program }) = err else { panic!("expected Run(NotFound)") };
    assert_eq!(program, OsString::from("git"));
}

#[test]
fn exit_carries_args_code_and_stderr() {
    let err = GitError::Exit { args: vec!["-C".into(), "/r".into()], code: None, stderr: b"boom".to_vec() };
    let GitError::Exit { args, code, stderr } = err else { panic!("expected Exit") };
    assert_eq!(args, [OsString::from("-C"), OsString::from("/r")]);
    assert_eq!(code, None);
    assert_eq!(stderr, b"boom");
}

#[test]
fn not_a_repository_carries_the_repo() {
    let GitError::NotARepository { repo } = (GitError::NotARepository { repo: "/tmp/x".into() }) else {
        panic!("expected NotARepository")
    };
    assert_eq!(repo, PathBuf::from("/tmp/x"));
}

#[test]
fn bad_output_carries_args_and_stdout() {
    let err = GitError::BadOutput { args: vec!["rev-parse".into()], stdout: b"??".to_vec() };
    let GitError::BadOutput { args, stdout } = err else { panic!("expected BadOutput") };
    assert_eq!(args, [OsString::from("rev-parse")]);
    assert_eq!(stdout, b"??");
}
