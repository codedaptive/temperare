//! engines: one adapter per engine: run it, read its output into findings; "not available" when it is absent. See the Temperare mission (Layout).

pub mod ast_grep;
pub mod clippy;
pub mod ruff;
pub mod swiftlint;
pub mod tessera;

mod availability;
mod check_run;
mod engine_hits;
mod findings_from;
mod host;
mod parse_failed;
mod printed_start;
mod program;
mod ran;
mod rel_to_folder;
mod rule_folder;
mod run_rules;
mod run_rules_counted;
mod run_rules_counted_with;
mod types;

pub use availability::availability;
pub use run_rules::run_rules;
pub use run_rules_counted::run_rules_counted;
pub use types::{Availability, Hit, ParamError, ParseError, RuleRun};

#[cfg(test)]
mod availability_tests;
#[cfg(test)]
mod check_run_tests;
#[cfg(test)]
mod engine_hits_tests;
#[cfg(test)]
mod findings_from_tests;
#[cfg(test)]
mod host_tests;
#[cfg(test)]
mod integration;
#[cfg(test)]
mod parse_failed_tests;
#[cfg(test)]
mod printed_start_tests;
#[cfg(test)]
mod program_tests;
#[cfg(test)]
mod ran_tests;
#[cfg(test)]
mod rel_to_folder_tests;
