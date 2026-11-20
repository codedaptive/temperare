//! ZUT for ran.rs: each way the host's run can end, read as output or a `Stop`. The host is a fake; nothing runs.

use crate::host::Host;
use crate::ran::ran;
use crate::types::Stop;
use procio::{Cmd, Output, ProcError};
use std::cell::Cell;
use std::ffi::OsString;
use std::io;
use std::time::Duration;

/// `ran(cmd, host)` with a host whose run gives `answer` and whose PATH lookup is never asked.
fn ran_with(cmd: &Cmd, answer: impl Fn() -> Result<Output, ProcError>) -> Result<Output, Stop> {
    let run = |_: &Cmd| answer();
    let on_path = |_: &str| -> bool { panic!("ran looks nothing up on PATH") };
    ran(cmd, &Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None) })
}

#[test]
fn any_exit_code_passes_the_output_through() {
    let out = Output { code: Some(3), stdout: b"out".to_vec(), stderr: b"err".to_vec() };
    assert_eq!(ran_with(&Cmd::new("ruff"), || Ok(out.clone())), Ok(out.clone()));
}

#[test]
fn the_host_runs_the_cmd_it_is_given() {
    let cmd = Cmd { args: vec![OsString::from("check")], ..Cmd::new("ruff") };
    let seen = std::cell::RefCell::new(None);
    let run = |given: &Cmd| {
        *seen.borrow_mut() = Some(given.clone());
        Ok(Output { code: Some(0), stdout: Vec::new(), stderr: Vec::new() })
    };
    let on_path = |_: &str| false;
    ran(&cmd, &Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None) }).unwrap();
    assert_eq!(seen.into_inner(), Some(cmd));
}

#[test]
fn a_program_not_found_is_not_available_with_availabilitys_text() {
    let got = ran_with(&Cmd::new("swiftlint"), || Err(ProcError::NotFound { program: "swiftlint".into() }));
    assert_eq!(got, Err(Stop::NotAvailable { why: "swiftlint not found on PATH".to_string() }));
}

#[test]
fn a_program_that_could_not_run_failed_with_no_code_naming_it() {
    let got = ran_with(&Cmd::new("ruff"), || {
        Err(ProcError::Spawn { program: "ruff".into(), io: io::Error::other("permission denied") })
    });
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: "ruff: could not run: permission denied".to_string() }));
}

#[test]
fn a_program_that_timed_out_failed_with_no_code_naming_it() {
    let got = ran_with(&Cmd::new("ast-grep"), || {
        Err(ProcError::TimedOut { program: "ast-grep".into(), after: Duration::from_secs(2) })
    });
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: "ast-grep: timed out after 2s".to_string() }));
}
