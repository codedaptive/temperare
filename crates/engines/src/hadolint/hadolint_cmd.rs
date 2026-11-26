//! `hadolint_cmd`: the hadolint run for one rule, as a command.

use crate::entry::{entry_cmd, Entry};
use procio::Cmd;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

/// `entry_cmd`'s command for `check` in `cwd` (the program, `entry.args`, `--error=<check>`), then
/// `--config=<config>` as one argument, then `--`, then each of `files` as given, relative to `cwd`, with each of
/// `removed` removed from its environment. The caller names the variables (`run_hadolint` passes every `HADOLINT_*`
/// in tm's environment); this reads no environment. The `--` keeps a file named `-x.Dockerfile` from being read as an
/// option.
pub(crate) fn hadolint_cmd(
    entry: &Entry,
    check: &str,
    config: &Path,
    files: &[PathBuf],
    cwd: &Path,
    removed: &[OsString],
) -> Cmd {
    let mut cmd = entry_cmd(entry, OsStr::new(check), &[], None, cwd);
    let mut config_arg = OsString::from("--config=");
    config_arg.push(config);
    cmd.args.extend([config_arg, OsString::from("--")]);
    cmd.args.extend(files.iter().map(|file| file.clone().into_os_string()));
    cmd.env_remove.extend_from_slice(removed);
    cmd
}
