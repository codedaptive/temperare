//! `tracked_files`: the tracked files, leaving out those missing from the work tree and every skip-worktree entry.

use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

use crate::git::parse::{parse_nul_list, path_from_bytes};
use crate::git::{git_checked, git_cmd, GitError};
use crate::{run, Cmd, Output};

/// The tracked files under `repo`, in git's order, each listed once and relative to the top of the repository,
/// leaving out those missing from the work tree and every skip-worktree entry.
///
/// - Tracked only: the entries of the index. Untracked and ignored files are never listed.
/// - A tracked file deleted from the work tree is dropped. A skip-worktree entry is dropped whether or not its file
///   is present: git does not check the work tree for it (git-update-index's skip-worktree rule), so its presence
///   is unknown. An unmerged file is listed once.
/// - Each path is relative to the top of the repository (`--full-name`), even when `repo` is a subdirectory.
/// - `pathspecs` follow `--` exactly as given and git reads them relative to `repo`. Wildcards such as `*.sh`, and
///   magic such as `:/`, work because `git_cmd` clears `GIT_LITERAL_PATHSPECS`; `--` keeps any pathspec from being read as an option.
/// - An empty `pathspecs` lists every tracked file under `repo`; a pathspec matching nothing gives an empty list.
/// - Errors are `GitError`, as for the other operations. `ls-files` is local: it reaches no network.
pub fn tracked_files<S: AsRef<OsStr>>(repo: &Path, pathspecs: &[S]) -> Result<Vec<PathBuf>, GitError> {
    let cmd = git_cmd(repo, tracked_files_args(pathspecs));
    let out = run(&cmd)?;
    tracked_files_answer(&cmd, out)
}

/// `-z` keeps each path's bytes exactly, with no quoting of unusual names. `-t` tags each entry with its status, so
/// the answer can tell a deleted, skip-worktree or unmerged entry from a cached one. `--cached` lists the index
/// entries; `--deleted` adds a second, `R`-tagged entry for each one missing from the work tree, so it can be dropped.
/// `--full-name` makes each path relative to the top of the repository. `--` ends the options, so every pathspec
/// is read as a pathspec.
pub(crate) fn tracked_files_args<S: AsRef<OsStr>>(pathspecs: &[S]) -> Vec<OsString> {
    ["ls-files", "-z", "-t", "--cached", "--deleted", "--full-name", "--"]
        .into_iter()
        .map(OsString::from)
        .chain(pathspecs.iter().map(|spec| spec.as_ref().to_os_string()))
        .collect()
}

/// Keeps the `H` (cached) and `M` (unmerged) entries whose path has no `R` (deleted) entry, each path once at its
/// first occurrence. `S` (skip-worktree) entries and entries without the `X ` shape are dropped.
pub(crate) fn tracked_files_answer(cmd: &Cmd, out: Output) -> Result<Vec<PathBuf>, GitError> {
    let out = git_checked(cmd, out, &[])?;
    let entries: Vec<(u8, Vec<u8>)> = parse_nul_list(&out.stdout).into_iter().filter_map(split_tag).collect();
    let deleted: HashSet<&[u8]> =
        entries.iter().filter(|(tag, _)| *tag == b'R').map(|(_, path)| path.as_slice()).collect();
    let mut seen = HashSet::new();
    Ok(entries
        .iter()
        .filter(|(tag, path)| matches!(tag, b'H' | b'M') && !deleted.contains(path.as_slice()) && seen.insert(path))
        .map(|(_, path)| path_from_bytes(path.clone()))
        .collect())
}

/// An entry of `ls-files -t` split into its one-byte tag and its path; `None` when it is not a tag, a space, then
/// the path.
fn split_tag(entry: Vec<u8>) -> Option<(u8, Vec<u8>)> {
    match entry.as_slice() {
        [tag, b' ', path @ ..] => Some((*tag, path.to_vec())),
        _ => None,
    }
}
