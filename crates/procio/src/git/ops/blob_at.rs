//! `blob_at`: a file's bytes as a commit holds them.

use std::ffi::OsString;
use std::path::Path;

use super::exit_128_saying::exit_128_saying;
use crate::git::{git_checked, git_cmd, GitError};
use crate::{run, Cmd, Output};

/// The bytes of `path` at `rev`, exactly as committed, or `None` when `rev` holds no such path. A `path` that starts
/// with `./` or `../` is relative to `repo` (git runs in `repo`); any other path is relative to the top of the
/// repository. A `rev` that names nothing is `GitError::Exit`.
pub fn blob_at(repo: &Path, rev: &str, path: &Path) -> Result<Option<Vec<u8>>, GitError> {
    let cmd = git_cmd(repo, blob_at_args(rev, path));
    let out = run(&cmd)?;
    blob_at_answer(&cmd, out)
}

/// `<rev>:<path>` is one argument with the path's bytes kept as given. No `--` goes before it: `show` would read
/// what comes after a `--` as a pathspec, not as the object to show.
pub(crate) fn blob_at_args(rev: &str, path: &Path) -> Vec<OsString> {
    let mut object = OsString::from(rev);
    object.push(":");
    object.push(path);
    vec!["show".into(), "--end-of-options".into(), object]
}

/// A missing path fails with exit 128 and one of two messages ("does not exist in", or "exists on disk, but not
/// in" when the working tree has the file); a missing rev says "invalid object name" under the same code and stays
/// an `Exit`.
pub(crate) fn blob_at_answer(cmd: &Cmd, out: Output) -> Result<Option<Vec<u8>>, GitError> {
    if exit_128_saying(&out, &[b"does not exist in", b"exists on disk, but not in"]) {
        return Ok(None);
    }
    Ok(Some(git_checked(cmd, out, &[])?.stdout))
}
