//! `GitError` and its trait impls, one impl per file.

mod git_error;
mod git_error_display;
mod git_error_from;
mod git_error_source;

pub use git_error::GitError;

#[cfg(test)]
mod git_error_display_tests;
#[cfg(test)]
mod git_error_from_tests;
#[cfg(test)]
mod git_error_source_tests;
#[cfg(test)]
mod git_error_tests;
