//! The swiftlint adapter: the swiftlint entry (its `swiftlint lint` command, the check it refuses, swiftlint's
//! unknown-rule warning) and the reader of its JSON output.

mod parse_swiftlint;
mod swiftlint_entry;
mod swiftlint_refusal;
mod unknown_rule;

#[cfg(test)]
mod integration;
#[cfg(test)]
mod parse_swiftlint_tests;
#[cfg(test)]
mod swiftlint_entry_tests;
#[cfg(test)]
mod swiftlint_refusal_tests;
#[cfg(test)]
mod unknown_rule_tests;

pub use parse_swiftlint::parse_swiftlint;
pub use swiftlint_entry::SWIFTLINT;
pub(crate) use swiftlint_refusal::swiftlint_refusal;
pub(crate) use unknown_rule::unknown_rule;
