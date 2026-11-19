//! procio: the one place that runs another program, git included. Depends on nothing.

mod cmd;
mod cmd_new;
mod command_from;
mod git;
#[cfg(unix)]
mod kill_group_cmd;
mod output;
mod proc_error;
mod proc_error_display;
mod proc_error_source;
mod run;
mod spawn_error;
mod spawn_reader;
mod spawn_writer;
mod wait_deadline;
mod which;

pub use cmd::Cmd;
pub use git::{added_lines, blob_at, changed_files, rev_parse, toplevel, untracked_files, GitError, Oid};
pub use output::Output;
pub use proc_error::ProcError;
pub use run::run;
pub use which::which;

#[cfg(test)]
mod cmd_new_tests;
#[cfg(test)]
mod cmd_tests;
#[cfg(test)]
mod command_from_tests;
#[cfg(all(test, unix))]
mod kill_group_cmd_tests;
#[cfg(test)]
mod output_tests;
#[cfg(test)]
mod proc_error_display_tests;
#[cfg(test)]
mod proc_error_source_tests;
#[cfg(test)]
mod proc_error_tests;
#[cfg(test)]
mod spawn_error_tests;
#[cfg(test)]
mod spawn_reader_tests;
#[cfg(test)]
mod spawn_writer_tests;
#[cfg(test)]
mod wait_deadline_tests;
#[cfg(test)]
mod which_tests;
