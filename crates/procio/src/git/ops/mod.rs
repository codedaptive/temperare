//! The six git operations, each a thin wrapper over `run` with its argument list and its reading of git's answer
//! kept as pure helpers beside it.

mod added_lines;
mod blob_at;
mod exit_128_saying;
mod rev_parse;
mod toplevel;
mod tracked_files;
mod untracked_files;

pub use added_lines::added_lines;
pub use blob_at::blob_at;
pub use rev_parse::rev_parse;
pub use toplevel::toplevel;
pub use tracked_files::tracked_files;
pub use untracked_files::untracked_files;

#[cfg(test)]
mod added_lines_tests;
#[cfg(test)]
mod blob_at_tests;
#[cfg(test)]
mod exit_128_saying_tests;
#[cfg(test)]
mod rev_parse_tests;
#[cfg(test)]
mod toplevel_tests;
#[cfg(test)]
mod tracked_files_tests;
#[cfg(test)]
mod untracked_files_tests;
