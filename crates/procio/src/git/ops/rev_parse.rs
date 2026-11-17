//! `rev_parse`: the commit a revision names.

use std::ffi::OsString;
use std::path::Path;

use crate::git::{git_checked, git_cmd, GitError, Oid};
use crate::{run, Cmd, Output};

/// The id of the commit `rev` names in `repo`, or `None` when it names no commit. `rev` is always read as a
/// revision, never as an option, whatever it starts with.
pub fn rev_parse(repo: &Path, rev: &str) -> Result<Option<Oid>, GitError> {
    let cmd = git_cmd(repo, rev_parse_args(rev));
    let out = run(&cmd)?;
    rev_parse_answer(&cmd, out)
}

/// `^{commit}` makes a tag resolve to its commit and anything that is not a commit fail.
pub(crate) fn rev_parse_args(rev: &str) -> Vec<OsString> {
    vec![
        "rev-parse".into(),
        "--verify".into(),
        "-q".into(),
        "--end-of-options".into(),
        format!("{rev}^{{commit}}").into(),
    ]
}

/// Exit 1 is `--verify -q`'s quiet "no such commit".
pub(crate) fn rev_parse_answer(cmd: &Cmd, out: Output) -> Result<Option<Oid>, GitError> {
    let out = git_checked(cmd, out, &[1])?;
    if out.code == Some(1) {
        return Ok(None);
    }
    let line = out.stdout.strip_suffix(b"\n").unwrap_or(&out.stdout);
    let oid = std::str::from_utf8(line).ok().and_then(Oid::parse);
    match oid {
        Some(oid) => Ok(Some(oid)),
        None => Err(GitError::BadOutput { args: cmd.args.clone(), stdout: out.stdout }),
    }
}
