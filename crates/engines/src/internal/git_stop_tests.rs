//! Unit test for git_stop.rs: each kind of `VcsError`, built by hand, with git on PATH or not, read as its `Stop`. Nothing
//! runs.

use crate::git_stop::git_stop;
use crate::types::Stop;
use ccvcslib::{VcsError, VcsFailure};
use std::path::PathBuf;

fn git_absent(program: &'static str) -> Option<PathBuf> {
    assert_eq!(program, "git");
    None
}

fn git_present(program: &'static str) -> Option<PathBuf> {
    assert_eq!(program, "git");
    Some(PathBuf::from("/usr/bin").join(program))
}

fn never_asked(program: &'static str) -> Option<PathBuf> {
    panic!("which asked for {program}")
}

fn failure(message: &str, exit_status: Option<i32>) -> VcsError {
    VcsError::Failed(VcsFailure { message: message.to_string(), exit_status })
}

fn failed(code: Option<i32>, stderr: &str) -> Stop {
    Stop::Failed { code, stderr: stderr.to_string() }
}

#[test]
fn git_stop_reads_each_error_as_its_stop() {
    let cases: Vec<(&str, VcsError, fn(&'static str) -> Option<PathBuf>, Stop)> = vec![
        ("unavailable, git absent", VcsError::Unavailable, git_absent, Stop::NotOnPath { program: "git".to_string() }),
        ("unavailable, git present", VcsError::Unavailable, git_present, failed(None, "version control unavailable")),
        ("not a working copy", VcsError::NotAWorkingCopy, never_asked, failed(None, "not a working copy")),
        ("exit with a code", failure("fatal: x", Some(128)), git_present, failed(Some(128), "fatal: x")),
        ("failed with no code", failure("unexpected output", None), git_present, failed(None, "unexpected output")),
        ("which not asked", failure("fatal: y", Some(1)), never_asked, failed(Some(1), "fatal: y")),
    ];
    let mut failed_rows = Vec::new();
    for (name, err, which, expected) in cases {
        let result = std::panic::catch_unwind(|| assert_eq!(git_stop(err, which), expected, "{name}"));
        if result.is_err() {
            failed_rows.push(name);
        }
    }
    assert!(failed_rows.is_empty(), "failed rows: {failed_rows:?}");
}
