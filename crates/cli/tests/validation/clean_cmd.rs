//! `clean_cmd`: a program to run for acceptance, described for procio, in an environment of exactly the given
//! variables: every inherited name is removed, then the given ones are added (procio applies `env_remove` before
//! `env`).

use std::ffi::OsString;
use std::path::Path;

use procio::Cmd;

/// `program args` in `cwd`, with only `vars` set.
pub fn clean_cmd(program: OsString, args: &[&str], cwd: &Path, vars: Vec<(String, String)>) -> Cmd {
    Cmd {
        args: args.iter().map(OsString::from).collect(),
        cwd: Some(cwd.to_path_buf()),
        env_remove: std::env::vars_os().map(|(name, _)| name).collect(),
        env: vars.into_iter().map(|(k, v)| (OsString::from(k), OsString::from(v))).collect(),
        ..Cmd::new(program)
    }
}
