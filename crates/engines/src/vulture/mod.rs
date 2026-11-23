//! The vulture adapter: the vulture entry (its command, the checks it refuses), the table of vulture's codes and
//! their messages, its flow with Python's warnings silenced and what git ignores left out (the ignored Python paths
//! git lists, and the `--exclude` argument made from them), and the reader of its text output.

mod exclude_arg;
mod ignored_entries;
mod ignored_python;
mod parse_vulture;
mod run_vulture;
mod run_vulture_with;
mod vulture_entry;
mod vulture_kinds;
mod vulture_refusal;

#[cfg(test)]
mod exclude_arg_tests;
#[cfg(test)]
mod ignored_entries_tests;
#[cfg(test)]
mod parse_vulture_tests;
#[cfg(test)]
mod run_vulture_with_tests;
#[cfg(test)]
mod vulture_entry_tests;
#[cfg(test)]
mod vulture_refusal_tests;

pub(crate) use exclude_arg::exclude_arg;
pub(crate) use ignored_entries::ignored_entries;
pub(crate) use ignored_python::ignored_python;
pub use parse_vulture::parse_vulture;
pub(crate) use run_vulture::run_vulture;
pub(crate) use run_vulture_with::run_vulture_with;
pub use vulture_entry::VULTURE;
pub(crate) use vulture_kinds::VULTURE_KINDS;
pub(crate) use vulture_refusal::vulture_refusal;
