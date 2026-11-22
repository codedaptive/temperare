//! The vulture adapter: the vulture entry (its command, the checks it refuses), the table of vulture's codes and
//! their messages, its flow with Python's warnings silenced, and the reader of its text output.

mod parse_vulture;
mod run_vulture;
mod vulture_entry;
mod vulture_kinds;
mod vulture_refusal;

#[cfg(test)]
mod parse_vulture_tests;
#[cfg(test)]
mod run_vulture_tests;
#[cfg(test)]
mod vulture_entry_tests;
#[cfg(test)]
mod vulture_refusal_tests;

pub use parse_vulture::parse_vulture;
pub(crate) use run_vulture::run_vulture;
pub use vulture_entry::VULTURE;
pub(crate) use vulture_kinds::VULTURE_KINDS;
pub(crate) use vulture_refusal::vulture_refusal;
