//! The ruff adapter: builds the `ruff check` command, runs it for a rule and reads its JSON output into hits.

mod hits_ruff;
mod parse_ruff;
mod ruff_cmd;

#[cfg(test)]
mod hits_ruff_tests;
#[cfg(test)]
mod parse_ruff_tests;
#[cfg(test)]
mod ruff_cmd_tests;

pub(crate) use hits_ruff::hits_ruff;
pub use parse_ruff::parse_ruff;
pub use ruff_cmd::ruff_cmd;
