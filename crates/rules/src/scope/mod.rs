//! Scoping a rule to files, and the rule and exception types.

mod exception;
mod rule;
mod rule_applies;
#[allow(clippy::module_inception)]
mod scope;
mod scope_build;
mod scope_matches;

#[cfg(test)]
mod exception_tests;
#[cfg(test)]
mod rule_applies_tests;
#[cfg(test)]
mod rule_tests;
#[cfg(test)]
mod scope_build_tests;
#[cfg(test)]
mod scope_matches_tests;
#[cfg(test)]
mod scope_tests;

pub use exception::Exception;
pub use rule::Rule;
pub use scope::Scope;
