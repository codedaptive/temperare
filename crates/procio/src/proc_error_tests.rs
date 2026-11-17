use std::ffi::OsString;
use std::io;
use std::time::Duration;

use crate::ProcError;

#[test]
fn not_found_carries_the_program() {
    let err = ProcError::NotFound { program: "swiftlint".into() };
    let ProcError::NotFound { program } = err else { panic!("expected NotFound") };
    assert_eq!(program, OsString::from("swiftlint"));
}

#[test]
fn spawn_carries_the_program_and_the_io_error() {
    let err = ProcError::Spawn { program: "git".into(), io: io::Error::from(io::ErrorKind::PermissionDenied) };
    let ProcError::Spawn { program, io } = err else { panic!("expected Spawn") };
    assert_eq!(program, OsString::from("git"));
    assert_eq!(io.kind(), io::ErrorKind::PermissionDenied);
}

#[test]
fn timed_out_carries_the_program_and_the_timeout() {
    let err = ProcError::TimedOut { program: "ruff".into(), after: Duration::from_secs(5) };
    let ProcError::TimedOut { program, after } = err else { panic!("expected TimedOut") };
    assert_eq!(program, OsString::from("ruff"));
    assert_eq!(after, Duration::from_secs(5));
}
