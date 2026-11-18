//! Runs a program for acceptance through procio, the one place that starts programs (S-26), in an environment of
//! exactly the given variables (`clean_cmd`).

use std::ffi::OsString;
use std::path::Path;

use procio::{run, Output};

use crate::clean_cmd::clean_cmd;

/// `program args` in `cwd`, with only `vars` set; panics when it cannot start or a signal ends it.
pub fn run_clean(program: impl Into<OsString>, args: &[&str], cwd: &Path, vars: Vec<(String, String)>) -> Output {
    let program = program.into();
    let out = run(&clean_cmd(program.clone(), args, cwd, vars))
        .unwrap_or_else(|e| panic!("could not run {program:?} {args:?}: {e}"));
    assert!(out.code.is_some(), "{program:?} {args:?} was ended by a signal");
    out
}
