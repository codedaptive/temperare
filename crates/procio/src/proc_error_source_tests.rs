use std::error::Error;
use std::io;
use std::time::Duration;

use crate::ProcError;

#[test]
fn spawn_has_the_io_error_as_source() {
    let err = ProcError::Spawn { program: "git".into(), io: io::Error::other("pipe closed") };
    assert_eq!(err.source().map(ToString::to_string), Some("pipe closed".to_string()));
}

#[test]
fn not_found_has_no_source() {
    assert!(ProcError::NotFound { program: "git".into() }.source().is_none());
}

#[test]
fn timed_out_has_no_source() {
    assert!(ProcError::TimedOut { program: "git".into(), after: Duration::from_secs(1) }.source().is_none());
}
