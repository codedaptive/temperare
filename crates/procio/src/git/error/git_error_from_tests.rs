use std::ffi::OsString;
use std::time::Duration;

use super::GitError;
use crate::ProcError;

#[test]
fn a_proc_error_becomes_run_unchanged() {
    let err = GitError::from(ProcError::TimedOut { program: "git".into(), after: Duration::from_secs(3) });
    let GitError::Run(ProcError::TimedOut { program, after }) = err else { panic!("expected Run(TimedOut)") };
    assert_eq!(program, OsString::from("git"));
    assert_eq!(after, Duration::from_secs(3));
}

#[test]
fn question_mark_converts_a_proc_error() {
    fn op() -> Result<(), GitError> {
        Err(ProcError::NotFound { program: "git".into() })?
    }
    assert!(matches!(op(), Err(GitError::Run(ProcError::NotFound { .. }))));
}
