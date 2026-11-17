//! Runs a program for acceptance through procio, the one place that starts programs (S-26), in an environment of
//! exactly the given variables: every inherited name is removed, then the given ones are added (procio applies
//! `env_remove` before `env`).

use std::ffi::OsString;
use std::path::Path;

use procio::{run, Cmd, Output};

/// `program args` in `cwd`, with only `vars` set; panics when it cannot start or a signal ends it.
pub fn run_clean(program: impl Into<OsString>, args: &[&str], cwd: &Path, vars: Vec<(String, String)>) -> Output {
    let program = program.into();
    let cmd = Cmd {
        args: args.iter().map(OsString::from).collect(),
        cwd: Some(cwd.to_path_buf()),
        env_remove: std::env::vars_os().map(|(name, _)| name).collect(),
        env: vars.into_iter().map(|(k, v)| (OsString::from(k), OsString::from(v))).collect(),
        ..Cmd::new(program.clone())
    };
    let out = run(&cmd).unwrap_or_else(|e| panic!("could not run {program:?} {args:?}: {e}"));
    assert!(out.code.is_some(), "{program:?} {args:?} was ended by a signal");
    out
}
