//! `Display` for `GitError`: one line saying what git was asked and what went wrong.

use std::ffi::OsString;
use std::fmt;

use super::GitError;

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitError::Run(err) => write!(f, "{err}"),
            GitError::NotARepository { repo } => write!(f, "{}: not a git repository", repo.display()),
            GitError::Exit { args, code, stderr } => {
                write!(f, "git {}: ", joined(args))?;
                match code {
                    Some(code) => write!(f, "exit {code}")?,
                    None => f.write_str("killed by a signal")?,
                }
                // git ends its message with a newline; trimmed so the error stays one line.
                let stderr = String::from_utf8_lossy(stderr);
                match stderr.trim() {
                    "" => Ok(()),
                    message => write!(f, ": {message}"),
                }
            }
            GitError::BadOutput { args, .. } => write!(f, "git {}: unexpected output", joined(args)),
        }
    }
}

/// The arguments space-joined, any that are not UTF-8 written lossily.
fn joined(args: &[OsString]) -> String {
    args.iter().map(|a| a.to_string_lossy()).collect::<Vec<_>>().join(" ")
}
