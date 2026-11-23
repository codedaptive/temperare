//! `sqlfluff_cmd`: the sqlfluff run for one rule, as a command.

use crate::entry::{entry_cmd, Entry};
use procio::Cmd;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

/// Variables removed from the run's environment: `XDG_CONFIG_HOME` would point sqlfluff's user config search
/// (platformdirs) outside the repository, and `PYTHONPATH` and `PYTHONUSERBASE` would load Python packages (a plugin,
/// a templater) the repository does not name.
const REMOVED: [&str; 3] = ["XDG_CONFIG_HOME", "PYTHONPATH", "PYTHONUSERBASE"];

/// `entry_cmd`'s command for `check` and `config` in `cwd` (the program, `entry.args`, `--rules=<check>`,
/// `--config=<config>` when given), then `--`, then each of `files` as given, relative to `cwd`; the `--` keeps a
/// file named `-x.sql` from being read as an option. `HOME` is set to `home`, the repository's top, so the configs
/// sqlfluff reads from `~` and from the folders between `~` and each file are the repository's own; `REMOVED` are
/// taken out.
pub(crate) fn sqlfluff_cmd(
    entry: &Entry,
    check: &str,
    config: Option<&str>,
    files: &[PathBuf],
    cwd: &Path,
    home: &Path,
) -> Cmd {
    let mut cmd = entry_cmd(entry, OsStr::new(check), &[], config, cwd);
    cmd.args.push("--".into());
    cmd.args.extend(files.iter().map(|file| file.clone().into_os_string()));
    cmd.env.push(("HOME".into(), home.into()));
    cmd.env_remove.extend(REMOVED.map(OsString::from));
    cmd
}
