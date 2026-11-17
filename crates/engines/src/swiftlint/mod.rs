//! The swiftlint adapter: builds the `swiftlint lint` command, runs it for a rule and reads its JSON output into hits.

mod hits_swiftlint;
mod parse_swiftlint;
mod swiftlint_cmd;

#[cfg(test)]
mod hits_swiftlint_tests;
#[cfg(test)]
mod parse_swiftlint_tests;
#[cfg(test)]
mod swiftlint_cmd_tests;

pub(crate) use hits_swiftlint::hits_swiftlint;
pub use parse_swiftlint::parse_swiftlint;
pub use swiftlint_cmd::swiftlint_cmd;
