//! `git_checked`: whether git's exit code is an answer the operation takes.

use crate::git::GitError;
use crate::{Cmd, Output};

/// `out` when git exited 0 or with one of `answers` (the codes the operation reads as a result, such as 1 for
/// `rev-parse --verify -q` finding nothing); otherwise `GitError::Exit` with the command's args, the code (None
/// for a signal) and git's stderr. Pure: it looks only at `out.code`.
pub(crate) fn git_checked(cmd: &Cmd, out: Output, answers: &[i32]) -> Result<Output, GitError> {
    match out.code {
        Some(code) if code == 0 || answers.contains(&code) => Ok(out),
        code => Err(GitError::Exit { args: cmd.args.clone(), code, stderr: out.stderr }),
    }
}
