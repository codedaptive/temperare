//! Scoping a rule to files, the rule and exception types, and the calendar day.

mod date;
mod date_new;
mod date_parse;
mod exception;
mod exception_new;
mod rule;
mod rule_applies;
mod rule_new;
#[allow(clippy::module_inception)]
mod scope;
mod scope_build;
mod scope_matches;

#[cfg(test)]
mod date_new_tests;
#[cfg(test)]
mod date_parse_tests;
#[cfg(test)]
mod date_tests;
#[cfg(test)]
mod exception_new_tests;
#[cfg(test)]
mod exception_tests;
#[cfg(test)]
mod rule_applies_tests;
#[cfg(test)]
mod rule_new_tests;
#[cfg(test)]
mod rule_tests;
#[cfg(test)]
mod scope_build_tests;
#[cfg(test)]
mod scope_matches_tests;
#[cfg(test)]
mod scope_tests;

pub use date::Date;
pub use exception::Exception;
pub use rule::Rule;
pub use scope::Scope;
