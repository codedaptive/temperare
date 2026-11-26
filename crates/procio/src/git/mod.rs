//! git: the command builder every operation shares; each operation is a thin wrapper over `run` and a pure parser
//! of what git printed.

mod command;
mod error;
mod oid;
mod ops;
mod parse;

pub use command::git_cmd;
pub use error::GitError;
pub use oid::Oid;
pub use ops::{added_lines, blob_at, rev_parse, toplevel, tracked_files, untracked_files};

pub(crate) use command::git_checked;
