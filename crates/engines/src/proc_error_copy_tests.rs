//! Unit tests for proc_error_copy.rs: each variant copied with its program and what it carries.

use crate::proc_error_copy::proc_error_copy;
use procio::ProcError;
use std::io;
use std::time::Duration;

#[test]
fn not_found_keeps_its_program() {
    let copy = proc_error_copy(&ProcError::NotFound { program: "ruff".into() });
    assert!(matches!(&copy, ProcError::NotFound { program } if program == "ruff"), "{copy:?}");
}

#[test]
fn spawn_keeps_its_program_and_the_io_errors_kind_and_message() {
    let err = ProcError::Spawn { program: "cargo".into(), io: io::Error::new(io::ErrorKind::BrokenPipe, "pipe closed") };
    let copy = proc_error_copy(&err);
    let ProcError::Spawn { program, io } = &copy else { panic!("not Spawn: {copy:?}") };
    assert_eq!(program, "cargo");
    assert_eq!(io.kind(), io::ErrorKind::BrokenPipe);
    assert_eq!(copy.to_string(), "cargo: could not run: pipe closed");
}

#[test]
fn spawn_from_an_os_error_keeps_its_words() {
    let os = io::Error::from_raw_os_error(13);
    let (kind, words) = (os.kind(), os.to_string());
    let copy = proc_error_copy(&ProcError::Spawn { program: "swiftlint".into(), io: os });
    let ProcError::Spawn { program, io } = &copy else { panic!("not Spawn: {copy:?}") };
    assert_eq!(program, "swiftlint");
    assert_eq!(io.kind(), kind);
    assert_eq!(io.to_string(), words);
}

#[test]
fn timed_out_keeps_its_program_and_after() {
    let copy = proc_error_copy(&ProcError::TimedOut { program: "ast-grep".into(), after: Duration::from_secs(90) });
    assert!(
        matches!(&copy, ProcError::TimedOut { program, after } if program == "ast-grep" && *after == Duration::from_secs(90)),
        "{copy:?}"
    );
}
