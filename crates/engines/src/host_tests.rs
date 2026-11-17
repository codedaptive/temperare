//! ZUT for host.rs: a Host built from closures calls them, giving their answers back unchanged.

use crate::host::Host;
use procio::{Cmd, Output, ProcError};
use std::cell::RefCell;

#[test]
fn run_calls_the_run_closure_with_the_cmd_and_returns_its_output() {
    let seen = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        seen.borrow_mut().push(cmd.program.clone());
        Ok(Output { code: Some(7), stdout: b"out".to_vec(), stderr: b"err".to_vec() })
    };
    let on_path = |_: &str| false;
    let host = Host { run: &run, on_path: &on_path };
    let got = (host.run)(&Cmd::new("ruff")).unwrap();
    assert_eq!(got, Output { code: Some(7), stdout: b"out".to_vec(), stderr: b"err".to_vec() });
    assert_eq!(seen.into_inner(), vec!["ruff"]);
}

#[test]
fn on_path_calls_the_on_path_closure_and_returns_its_answer() {
    let run = |_: &Cmd| -> Result<Output, ProcError> { panic!("on_path runs nothing") };
    let on_path = |name: &str| name == "ruff";
    let host = Host { run: &run, on_path: &on_path };
    assert!((host.on_path)("ruff"));
    assert!(!(host.on_path)("swiftlint"));
}
