//! `mypy_cmd`: the mypy run for one rule, as a command.

use crate::entry::{entry_cmd, Entry};
use procio::Cmd;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

/// `entry_cmd`'s command for `check` in `cwd` (the program, `entry.args`, `--enable-error-code=<check>`), then
/// `--config-file=<config>` as one argument, or `--config-file=` when `config` is None, which makes mypy read no
/// config at all (not even the user's home one), then `--`, then each of `files` as given, relative to `cwd`. The
/// `--` keeps a file named `-x.py` from being read as an option.
pub(crate) fn mypy_cmd(entry: &Entry, check: &str, config: Option<&Path>, files: &[PathBuf], cwd: &Path) -> Cmd {
    let mut cmd = entry_cmd(entry, OsStr::new(check), &[], None, cwd);
    let mut config_arg = OsString::from("--config-file=");
    if let Some(config) = config {
        config_arg.push(config);
    }
    cmd.args.push(config_arg);
    cmd.args.push("--".into());
    cmd.args.extend(files.iter().map(|file| file.clone().into_os_string()));
    cmd
}
