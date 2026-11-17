use std::ffi::{OsStr, OsString};
use std::io;

use crate::spawn_error::spawn_error;
use crate::ProcError;

#[test]
fn no_such_file_and_not_found_is_not_found() {
    match spawn_error(OsStr::new("swiftlint"), io::Error::from(io::ErrorKind::NotFound), false) {
        ProcError::NotFound { program } => assert_eq!(program, OsString::from("swiftlint")),
        other => panic!("expected NotFound, got {other:?}"),
    }
}

#[test]
fn no_such_file_for_a_found_program_is_spawn() {
    match spawn_error(OsStr::new("git"), io::Error::from(io::ErrorKind::NotFound), true) {
        ProcError::Spawn { program, io } => {
            assert_eq!(program, OsString::from("git"));
            assert_eq!(io.kind(), io::ErrorKind::NotFound);
        }
        other => panic!("expected Spawn, got {other:?}"),
    }
}

#[test]
fn another_error_for_a_missing_program_is_spawn() {
    match spawn_error(OsStr::new("tool"), io::Error::from(io::ErrorKind::PermissionDenied), false) {
        ProcError::Spawn { program, io } => {
            assert_eq!(program, OsString::from("tool"));
            assert_eq!(io.kind(), io::ErrorKind::PermissionDenied);
        }
        other => panic!("expected Spawn, got {other:?}"),
    }
}
