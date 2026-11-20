//! ZUT for shell_files.rs with a fake host: the wrapper runs the tracked listing and keeps its scripts. Nothing runs,
//! and the listing has no extensionless path, so no file is read.
//!
//! ls_files_scripts.bin: stdout of `git ls-files -z -t --cached --deleted` (git 2.56.0) in a throwaway repository
//! tracking a.sh and b.sh, exit 0.

use super::shell_files;
use crate::host::Host;
use crate::types::Stop;
use procio::{Cmd, Output, ProcError};
use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

const FOLDER: &str = "/no/such/engines-zut-folder";

/// `shell_files` with git answering the fixture; also the commands run.
fn run() -> (Result<Vec<PathBuf>, Stop>, Vec<Cmd>) {
    let ran = RefCell::new(Vec::new());
    let host_run = |cmd: &Cmd| -> Result<Output, ProcError> {
        ran.borrow_mut().push(cmd.clone());
        Ok(Output { code: Some(0), stdout: include_bytes!("fixtures/ls_files_scripts.bin").to_vec(), stderr: Vec::new() })
    };
    let raw_chars = Cell::new(None);
    let got = shell_files(Path::new(FOLDER), &Host { run: &host_run, on_path: &|_| true, raw_chars: &raw_chars });
    (got, ran.into_inner())
}

#[test]
fn the_tracked_scripts_are_listed() {
    let (got, _) = run();
    assert_eq!(got, Ok(vec![PathBuf::from("a.sh"), PathBuf::from("b.sh")]));
}

#[test]
fn git_lists_the_tracked_files_in_the_folder() {
    let (_, ran) = run();
    assert_eq!(ran.len(), 1);
    assert_eq!(ran[0].program, OsString::from("git"));
    assert_eq!(ran[0].args, ["ls-files", "-z", "-t", "--cached", "--deleted"].map(OsString::from).to_vec());
    assert_eq!(ran[0].cwd, Some(PathBuf::from(FOLDER)));
}
