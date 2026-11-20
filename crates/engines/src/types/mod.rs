//! The adapters' shared value types: an engine's availability, one hit, the parse and param errors, why a rule
//! stops without findings, one rule as it was run, one engine's version, and a whole check's runs with its engines'
//! versions.

mod availability;
mod checked_rules;
mod engine_version;
mod hit;
mod param_error;
mod parse_error;
mod rule_run;
mod stop;

#[cfg(test)]
mod availability_tests;
#[cfg(test)]
mod checked_rules_tests;
#[cfg(test)]
mod engine_version_tests;
#[cfg(test)]
mod hit_tests;
#[cfg(test)]
mod param_error_tests;
#[cfg(test)]
mod parse_error_tests;
#[cfg(test)]
mod rule_run_tests;
#[cfg(test)]
mod stop_tests;

pub use availability::Availability;
pub use checked_rules::CheckedRules;
pub use engine_version::EngineVersion;
pub use hit::Hit;
pub use param_error::ParamError;
pub use parse_error::ParseError;
pub use rule_run::RuleRun;
pub(crate) use stop::Stop;
