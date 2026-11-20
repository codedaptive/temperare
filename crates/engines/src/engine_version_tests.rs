//! ZUT for engine_version.rs: one probe through a fake host, read by the entry's probe reader; nothing runs.

use crate::engine_version::engine_version;
use crate::host::Host;
use crate::ruff::RUFF;
use crate::swiftlint::SWIFTLINT;
use crate::tessera::TESSERA;
use procio::{Cmd, Output, ProcError};
use std::cell::{Cell, RefCell};
use std::path::Path;

const FOLDER: &str = "/no/such/engines-zut-folder";

fn printed(code: i32, stdout: &str) -> Result<Output, ProcError> {
    Ok(Output { code: Some(code), stdout: stdout.as_bytes().to_vec(), stderr: vec![] })
}

#[test]
fn the_probe_is_the_program_and_its_args_in_the_folder_and_its_word_is_the_version() {
    let seen = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| {
        seen.borrow_mut().push(cmd.clone());
        printed(0, "ruff 0.17.4\n")
    };
    let host = Host { run: &run, on_path: &|_| true, raw_chars: &Cell::new(None) };
    assert_eq!(engine_version(&RUFF, Path::new(FOLDER), &host), Some("0.17.4".to_string()));
    let seen = seen.into_inner();
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].program, "ruff");
    assert_eq!(seen[0].args, vec!["--version"]);
    assert_eq!(seen[0].cwd.as_deref(), Some(Path::new(FOLDER)));
}

#[test]
fn swiftlint_is_probed_with_its_own_args() {
    let seen = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| {
        seen.borrow_mut().push(cmd.clone());
        printed(0, "0.59.1\n")
    };
    let host = Host { run: &run, on_path: &|_| true, raw_chars: &Cell::new(None) };
    assert_eq!(engine_version(&SWIFTLINT, Path::new(FOLDER), &host), Some("0.59.1".to_string()));
    let seen = seen.into_inner();
    assert_eq!(seen[0].program, "swiftlint");
    assert_eq!(seen[0].args, vec!["version"]);
}

#[test]
fn a_probe_that_cannot_run_gives_none() {
    let not_found = |_: &Cmd| Err(ProcError::NotFound { program: "ruff".into() });
    let host = Host { run: &not_found, on_path: &|_| true, raw_chars: &Cell::new(None) };
    assert_eq!(engine_version(&RUFF, Path::new(FOLDER), &host), None);
}

#[test]
fn a_probe_that_fails_or_prints_no_version_gives_none() {
    let failed = |_: &Cmd| printed(2, "ruff 0.17.4\n");
    let host = Host { run: &failed, on_path: &|_| true, raw_chars: &Cell::new(None) };
    assert_eq!(engine_version(&RUFF, Path::new(FOLDER), &host), None);
    let wordless = |_: &Cmd| printed(0, "ruff unknown\n");
    let host = Host { run: &wordless, on_path: &|_| true, raw_chars: &Cell::new(None) };
    assert_eq!(engine_version(&RUFF, Path::new(FOLDER), &host), None);
}

#[test]
fn an_entry_with_no_probe_gives_none_and_runs_nothing() {
    let run = |cmd: &Cmd| -> Result<Output, ProcError> { panic!("no probe may run: {cmd:?}") };
    let host = Host { run: &run, on_path: &|_| true, raw_chars: &Cell::new(None) };
    assert_eq!(engine_version(&TESSERA, Path::new(FOLDER), &host), None);
}

#[test]
fn the_probe_leaves_the_count_as_it_was() {
    let run = |_: &Cmd| printed(0, &"9".repeat(1000));
    let raw_chars = Cell::new(Some(7));
    let host = Host { run: &run, on_path: &|_| true, raw_chars: &raw_chars };
    engine_version(&RUFF, Path::new(FOLDER), &host);
    assert_eq!(raw_chars.get(), Some(7));
}
