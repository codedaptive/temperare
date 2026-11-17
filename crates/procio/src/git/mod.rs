//! git: each operation is a thin wrapper over `run` and a pure parser of what git printed.

mod command;
mod error;
mod oid;
mod ops;
mod parse;

pub use error::GitError;
pub use oid::Oid;
pub use ops::{added_lines, blob_at, changed_files, rev_parse, toplevel, untracked_files};

pub(crate) use command::{git_checked, git_cmd};
