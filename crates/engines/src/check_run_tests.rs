//! ZUT for check_run.rs: the characters a check run printed, counted into the host, and runs that gave no output
//! leaving the count alone. The host is a fake; nothing runs.

use crate::check_run::check_run;
use crate::host::Host;
use crate::types::Stop;
use procio::{Cmd, Output, ProcError};
use std::cell::Cell;
use std::io;

/// `check_run` of a ruff command with a host whose run gives `answer`; also the count the host holds after it, which
/// starts as `start`.
fn counted(
    start: Option<usize>,
    answer: impl Fn() -> Result<Output, ProcError>,
) -> (Result<Output, Stop>, Option<usize>) {
    let run = |_: &Cmd| answer();
    let on_path = |_: &str| -> bool { panic!("check_run looks nothing up on PATH") };
    let raw_chars = Cell::new(start);
    let got = check_run(&Cmd::new("ruff"), &Host { run: &run, on_path: &on_path, raw_chars: &raw_chars, base: None });
    (got, raw_chars.get())
}

fn output(code: Option<i32>, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code, stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

#[test]
fn ascii_stdout_and_stderr_count_together_and_the_output_comes_back_unchanged() {
    let out = output(Some(1), b"[{\"a\":1}]", b"warn\n");
    let (got, chars) = counted(None, || Ok(out.clone()));
    assert_eq!(got, Ok(output(Some(1), b"[{\"a\":1}]", b"warn\n")));
    assert_eq!(chars, Some(14));
}

#[test]
fn a_non_ascii_character_counts_once() {
    // "né" is three bytes and two characters; "→" is three bytes and one character.
    let (_, chars) = counted(None, || Ok(output(Some(0), "né".as_bytes(), "→".as_bytes())));
    assert_eq!(chars, Some(3));
}

#[test]
fn an_invalid_utf8_byte_counts_once() {
    // b"\xff" is one byte and one replacement character; "é" is two bytes and one character: 3 bytes, 2 characters.
    let (got, chars) = counted(None, || Ok(output(Some(2), b"\xff", "é".as_bytes())));
    assert_eq!(got, Ok(output(Some(2), b"\xff", "é".as_bytes())));
    assert_eq!(chars, Some(2));
}

#[test]
fn empty_output_counts_zero() {
    let (_, chars) = counted(None, || Ok(output(Some(0), b"", b"")));
    assert_eq!(chars, Some(0));
}

#[test]
fn a_program_not_found_leaves_the_count_and_is_not_available() {
    let (got, chars) = counted(None, || Err(ProcError::NotFound { program: "ruff".into() }));
    assert_eq!(got, Err(Stop::NotAvailable { why: "ruff not found on PATH".to_string() }));
    assert_eq!(chars, None);
}

#[test]
fn a_program_that_could_not_run_leaves_the_count_and_failed() {
    let (got, chars) = counted(Some(7), || {
        Err(ProcError::Spawn { program: "ruff".into(), io: io::Error::other("permission denied") })
    });
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: "ruff: could not run: permission denied".to_string() }));
    assert_eq!(chars, Some(7));
}
