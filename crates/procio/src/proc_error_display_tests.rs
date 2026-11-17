use std::io;
use std::time::Duration;

use crate::ProcError;

#[test]
fn not_found_names_the_program() {
    assert_eq!(ProcError::NotFound { program: "swiftlint".into() }.to_string(), "swiftlint: not found");
}

#[test]
fn spawn_names_the_program_and_the_io_error() {
    let err = ProcError::Spawn { program: "git".into(), io: io::Error::other("pipe closed") };
    assert_eq!(err.to_string(), "git: could not run: pipe closed");
}

#[test]
fn timed_out_names_the_program_and_the_timeout() {
    let err = ProcError::TimedOut { program: "ruff".into(), after: Duration::from_millis(1500) };
    assert_eq!(err.to_string(), "ruff: timed out after 1.5s");
}
