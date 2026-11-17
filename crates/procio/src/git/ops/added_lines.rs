//! `added_lines`: the lines each file gained against a base commit.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::ops::Range;
use std::path::{Path, PathBuf};

use crate::git::parse::parse_added_lines;
use crate::git::{git_checked, git_cmd, GitError};
use crate::{run, Cmd, Output};

/// For each file whose text differs between `base` and the working tree and still exists, the lines added, as
/// 1-based half-open ranges of line numbers in the working-tree file. A file with only removed lines is present
/// with no ranges. Absent: deleted files, and those git shows without a text diff (binary, mode-only, new and
/// empty). Keys are relative to the top of the repository. Empty `paths` covers the whole repository; otherwise
/// only those paths are compared.
pub fn added_lines(
    repo: &Path,
    base: &str,
    paths: &[PathBuf],
) -> Result<BTreeMap<PathBuf, Vec<Range<u32>>>, GitError> {
    let cmd = git_cmd(repo, added_lines_args(base, paths));
    let out = run(&cmd)?;
    added_lines_answer(&cmd, out)
}

/// `--no-prefix` keeps the user's `diff.noprefix` and `diff.mnemonicPrefix` config from changing the header paths;
/// `--no-textconv` keeps a textconv driver from changing the line numbers.
pub(crate) fn added_lines_args(base: &str, paths: &[PathBuf]) -> Vec<OsString> {
    let mut args: Vec<OsString> = vec![
        "diff".into(),
        "-U0".into(),
        "--no-color".into(),
        "--no-renames".into(),
        "--no-ext-diff".into(),
        "--no-prefix".into(),
        "--no-textconv".into(),
        "--end-of-options".into(),
        base.into(),
        "--".into(),
    ];
    args.extend(paths.iter().map(|p| p.clone().into_os_string()));
    args
}

pub(crate) fn added_lines_answer(cmd: &Cmd, out: Output) -> Result<BTreeMap<PathBuf, Vec<Range<u32>>>, GitError> {
    let out = git_checked(cmd, out, &[])?;
    match parse_added_lines(&out.stdout) {
        Some(lines) => Ok(lines),
        None => Err(GitError::BadOutput { args: cmd.args.clone(), stdout: out.stdout }),
    }
}
