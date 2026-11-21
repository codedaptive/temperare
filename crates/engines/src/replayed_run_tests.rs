//! Unit tests for replayed_run.rs: one real run per key, its output or its error replayed to every later call of that key.

use crate::replayed_run::replayed_run;
use procio::{Cmd, Output, ProcError};
use std::cell::RefCell;
use std::ffi::OsString;

fn cmd(args: &[&str]) -> Cmd {
    Cmd { args: args.iter().map(OsString::from).collect(), ..Cmd::new("ruff") }
}

#[test]
fn a_key_runs_once_and_its_output_is_replayed() {
    let ran = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        ran.borrow_mut().push(cmd.args.clone());
        Ok(Output { code: Some(1), stdout: b"[]".to_vec(), stderr: b"warn".to_vec() })
    };
    let seen = RefCell::new(Vec::new());
    let help = cmd(&["-W", "help"]);
    let first = replayed_run(&seen, &help, &help, &run).unwrap();
    let second = replayed_run(&seen, &help, &help, &run).unwrap();
    let want = Output { code: Some(1), stdout: b"[]".to_vec(), stderr: b"warn".to_vec() };
    assert_eq!((first, second), (want.clone(), want));
    assert_eq!(ran.into_inner(), vec![vec![OsString::from("-W"), OsString::from("help")]]);
}

#[test]
fn another_key_runs_again() {
    let runs = RefCell::new(0);
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        *runs.borrow_mut() += 1;
        Ok(Output { code: Some(0), stdout: cmd.args[0].to_string_lossy().into_owned().into_bytes(), stderr: vec![] })
    };
    let seen = RefCell::new(Vec::new());
    let (a, b) = (cmd(&["a"]), cmd(&["b"]));
    assert_eq!(replayed_run(&seen, &a, &a, &run).unwrap().stdout, b"a");
    assert_eq!(replayed_run(&seen, &b, &b, &run).unwrap().stdout, b"b");
    assert_eq!(replayed_run(&seen, &a, &a, &run).unwrap().stdout, b"a");
    assert_eq!(runs.into_inner(), 2);
}

#[test]
fn the_command_run_is_cmd_and_it_is_stored_under_key() {
    let ran = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        ran.borrow_mut().push(cmd.args.clone());
        Ok(Output { code: Some(0), stdout: vec![], stderr: vec![] })
    };
    let seen = RefCell::new(Vec::new());
    let key = cmd(&["scan", "--rule", ""]);
    replayed_run(&seen, &key, &cmd(&["scan", "--rule", "/tmp/one.yml"]), &run).unwrap();
    replayed_run(&seen, &key, &cmd(&["scan", "--rule", "/tmp/two.yml"]), &run).unwrap();
    let only = vec![OsString::from("scan"), OsString::from("--rule"), OsString::from("/tmp/one.yml")];
    assert_eq!(ran.into_inner(), vec![only]);
}

#[test]
fn a_failed_run_is_replayed_as_the_same_error() {
    let runs = RefCell::new(0);
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        *runs.borrow_mut() += 1;
        Err(ProcError::NotFound { program: cmd.program.clone() })
    };
    let seen = RefCell::new(Vec::new());
    let check = cmd(&["--select=E711,F401"]);
    for _ in 0..2 {
        let err = replayed_run(&seen, &check, &check, &run).unwrap_err();
        assert!(matches!(&err, ProcError::NotFound { program } if program == "ruff"), "{err:?}");
    }
    assert_eq!(runs.into_inner(), 1);
}
