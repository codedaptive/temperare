//! The two pieces every git operation shares: building the command and judging its exit code.

mod git_checked;
mod git_cmd;

pub(crate) use git_checked::git_checked;
pub use git_cmd::git_cmd;

#[cfg(test)]
mod git_checked_tests;
#[cfg(test)]
mod git_cmd_tests;
