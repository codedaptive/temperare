//! The ruff adapter: the ruff entry (its `ruff check` command, the checks it refuses, ruff's unknown-selector words)
//! and the reader of its JSON output.

mod parse_ruff;
mod ruff_entry;
mod ruff_refusal;
mod ruff_selects;
mod unknown_selector;

#[cfg(test)]
mod parse_ruff_tests;
#[cfg(test)]
mod ruff_entry_tests;
#[cfg(test)]
mod ruff_refusal_tests;
#[cfg(test)]
mod ruff_selects_tests;
#[cfg(test)]
mod unknown_selector_tests;

pub use parse_ruff::parse_ruff;
pub use ruff_entry::RUFF;
pub(crate) use ruff_refusal::ruff_refusal;
pub(crate) use ruff_selects::ruff_selects;
pub(crate) use unknown_selector::unknown_selector;
