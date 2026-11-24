//! `hadolint_cmd`: the hadolint run for one rule, as a command.

use crate::entry::{entry_cmd, Entry};
use procio::Cmd;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

/// Every `HADOLINT_*` variable in the hadolint 2.15.1 binary. `HADOLINT_IGNORE` adds ignores outside the rule file
/// (measured); `HADOLINT_DISABLE_IGNORE_PRAGMA` switches the inline pragmas, `HADOLINT_FORMAT` sets the output format
/// (tm fixes it with `-f sarif`), `HADOLINT_NOFAIL` and `HADOLINT_VERBOSE` set the exit status and verbosity, and the
/// rest (by their names) set hadolint's levels, labels, registries and exit threshold. All are removed so a caller's
/// environment cannot change a rule's answer.
const HADOLINT_ENV: [&str; 13] = [
    "HADOLINT_DISABLE_IGNORE_PRAGMA",
    "HADOLINT_FAILURE_THRESHOLD",
    "HADOLINT_FORMAT",
    "HADOLINT_IGNORE",
    "HADOLINT_NOFAIL",
    "HADOLINT_OVERRIDE_ERROR",
    "HADOLINT_OVERRIDE_INFO",
    "HADOLINT_OVERRIDE_STYLE",
    "HADOLINT_OVERRIDE_WARNING",
    "HADOLINT_REQUIRE_LABELS",
    "HADOLINT_STRICT_LABELS",
    "HADOLINT_TRUSTED_REGISTRIES",
    "HADOLINT_VERBOSE",
];

/// `entry_cmd`'s command for `check` in `cwd` (the program, `entry.args`, `--error=<check>`), then
/// `--config=<config>` as one argument, then `--`, then each of `files` as given, relative to `cwd`, with every
/// `HADOLINT_*` variable hadolint reads removed from its environment. The `--` keeps a file named `-x.Dockerfile`
/// from being read as an option.
pub(crate) fn hadolint_cmd(entry: &Entry, check: &str, config: &Path, files: &[PathBuf], cwd: &Path) -> Cmd {
    let mut cmd = entry_cmd(entry, OsStr::new(check), &[], None, cwd);
    let mut config_arg = OsString::from("--config=");
    config_arg.push(config);
    cmd.args.extend([config_arg, OsString::from("--")]);
    cmd.args.extend(files.iter().map(|file| file.clone().into_os_string()));
    cmd.env_remove.extend(HADOLINT_ENV.map(OsString::from));
    cmd
}
