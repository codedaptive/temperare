//! engines: one adapter per engine: run it, read its output into findings; "not available" when it is absent. See the Temperare mission (Layout).

pub mod ast_grep;
pub mod clippy;
pub mod entry;
pub mod ruff;
pub mod sarif;
pub mod swiftlint;
pub mod tessera;

mod availability;
mod check_run;
mod engine_hits;
mod engine_version;
mod engine_versions;
mod entry_for;
mod findings_from;
mod host;
mod no_adapter_why;
mod parse_failed;
mod printed_start;
mod ran;
mod rel_to_folder;
mod rule_folder;
mod run_rules_counted;
mod run_rules_counted_with;
mod run_rules_versioned;
mod run_rules_versioned_with;
mod types;

pub use availability::availability;
pub use entry::{entry_cmd, Entry, ParamArg, Params, Pick, ReadFn, TempFile, VersionProbe};
pub use entry_for::entry_for;
pub use run_rules_counted::run_rules_counted;
pub use run_rules_versioned::run_rules_versioned;
pub use types::{Availability, CheckedRules, EngineVersion, Hit, ParamError, ParseError, RuleRun};

#[cfg(test)]
mod availability_tests;
#[cfg(test)]
mod check_run_tests;
#[cfg(test)]
mod engine_hits_tests;
#[cfg(test)]
mod engine_version_tests;
#[cfg(test)]
mod engine_versions_tests;
#[cfg(test)]
mod entry_for_tests;
#[cfg(test)]
mod findings_from_tests;
#[cfg(test)]
mod host_tests;
#[cfg(test)]
mod integration;
#[cfg(test)]
mod no_adapter_why_tests;
#[cfg(test)]
mod parse_failed_tests;
#[cfg(test)]
mod printed_start_tests;
#[cfg(test)]
mod ran_tests;
#[cfg(test)]
mod rel_to_folder_tests;
#[cfg(test)]
mod run_rules_counted_with_tests;
