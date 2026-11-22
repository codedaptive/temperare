//! The pylint adapter: the pylint entry (its json2 command, the checks it refuses, pylint's words for a message it
//! does not know) and the reader of its json2 output.

mod parse_pylint;
mod pylint_entry;
mod pylint_refusal;
mod unknown_message;

#[cfg(test)]
mod parse_pylint_tests;
#[cfg(test)]
mod pylint_entry_tests;
#[cfg(test)]
mod pylint_refusal_tests;
#[cfg(test)]
mod unknown_message_tests;

pub use parse_pylint::parse_pylint;
pub use pylint_entry::PYLINT;
pub(crate) use pylint_refusal::pylint_refusal;
pub(crate) use unknown_message::unknown_message;
