//! The ruff adapter: builds the `ruff check` command, runs it for a rule and reads its JSON output into hits.

mod hits_ruff;
mod parse_ruff;
mod ruff_cmd;
mod ruff_refusal;
mod ruff_selects;

#[cfg(test)]
mod hits_ruff_tests;
#[cfg(test)]
mod parse_ruff_tests;
#[cfg(test)]
mod ruff_cmd_tests;
#[cfg(test)]
mod ruff_refusal_tests;
#[cfg(test)]
mod ruff_selects_tests;

pub(crate) use hits_ruff::hits_ruff;
pub use parse_ruff::parse_ruff;
pub use ruff_cmd::ruff_cmd;
pub(crate) use ruff_refusal::ruff_refusal;
pub(crate) use ruff_selects::ruff_selects;
