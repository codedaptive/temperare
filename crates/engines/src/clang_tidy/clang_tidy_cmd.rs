//! `clang_tidy_cmd`: the clang-tidy run for one rule, as a command.

use crate::entry::{entry_cmd, Entry};
use procio::Cmd;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// `entry_cmd`'s command for `check` and `config` in `cwd` (the program, `entry.args`, the pick, the config argument,
/// `entry.after`), then `--config={}` when `config` is None, then each of `files` joined to `cwd`. `--config={}` makes
/// clang-tidy read no `.clang-tidy` file: the rule file is the only home for a rule, so a config tm was not given must
/// not change it. Each file is absolute, so a name beginning with `-` is never read as an option; no `--` follows,
/// since clang-tidy reads what comes after it as compile flags and would then bypass the compile database.
pub fn clang_tidy_cmd(entry: &Entry, check: &str, config: Option<&str>, files: &[PathBuf], cwd: &Path) -> Cmd {
    let mut cmd = entry_cmd(entry, OsStr::new(check), &[], config, cwd);
    if config.is_none() {
        cmd.args.push("--config={}".into());
    }
    cmd.args.extend(files.iter().map(|file| cwd.join(file).into_os_string()));
    cmd
}
