//! engines: one adapter per engine: run it, read its output into findings; "not available" when it is absent. See the Temperare mission (Layout).

pub mod ast_grep;
pub mod clippy;
pub mod entry;
pub mod git;
pub mod ruff;
pub mod sarif;
pub mod shellcheck;
pub mod swiftlint;
pub mod tessera;

mod availability;
mod change_of;
mod check_host;
mod check_run;
mod engine_hits;
mod engine_hits_with;
mod engine_version;
mod engine_versions;
mod entry_for;
mod files_in_folder;
mod findings_from;
mod folder_files;
mod host;
mod no_adapter_why;
mod no_file_match;
mod not_available_cause_of;
mod not_on_path_why;
mod parse_failed;
mod pick_at;
mod printed_start;
mod proc_error_copy;
mod ran;
mod refused;
mod rel_to_folder;
mod replayed_run;
mod rule_folder;
mod rule_host;
mod rule_run_of;
mod run_group;
mod run_rules_counted_with;
mod run_rules_listed;
mod run_rules_versioned_at;
mod run_rules_versioned_with;
mod shared_cmd;
mod shared_groups;
mod shares_with;
mod stopped;
mod types;
mod unavailable;
mod union_args;

pub use availability::availability;
pub use entry::{entry_cmd, Entry, ParamArg, Params, Pick, ReadFn, TempFile, VersionProbe};
pub use entry_for::entry_for;
pub use run_rules_versioned_at::run_rules_versioned_at;
pub use types::{Availability, CheckedRules, EngineVersion, Hit, NotAvailableCause, ParamError, ParseError, RuleRun};

#[cfg(test)]
mod availability_tests;
#[cfg(test)]
mod check_host_tests;
#[cfg(test)]
mod check_run_tests;
#[cfg(test)]
mod engine_hits_tests;
#[cfg(test)]
mod engine_hits_with_tests;
#[cfg(test)]
mod engine_version_tests;
#[cfg(test)]
mod engine_versions_tests;
#[cfg(test)]
mod entry_for_tests;
#[cfg(test)]
mod findings_from_tests;
#[cfg(test)]
mod folder_files_tests;
#[cfg(test)]
mod host_tests;
#[cfg(test)]
mod integration;
#[cfg(test)]
mod no_adapter_why_tests;
#[cfg(test)]
mod no_file_match_tests;
#[cfg(test)]
mod not_available_cause_of_tests;
#[cfg(test)]
mod not_on_path_why_tests;
#[cfg(test)]
mod parse_failed_tests;
#[cfg(test)]
mod pick_at_tests;
#[cfg(test)]
mod printed_start_tests;
#[cfg(test)]
mod proc_error_copy_tests;
#[cfg(test)]
mod ran_tests;
#[cfg(test)]
mod refused_tests;
#[cfg(test)]
mod rel_to_folder_tests;
#[cfg(test)]
mod replayed_run_tests;
#[cfg(test)]
mod rule_host_tests;
#[cfg(test)]
mod rule_run_of_tests;
#[cfg(test)]
mod run_group_tests;
#[cfg(test)]
mod run_rules_listed_tests;
#[cfg(test)]
mod run_rules_versioned_at_tests;
#[cfg(test)]
mod shared_cmd_tests;
#[cfg(test)]
mod shared_groups_tests;
#[cfg(test)]
mod shares_with_tests;
#[cfg(test)]
mod stopped_tests;
#[cfg(test)]
mod unavailable_tests;
#[cfg(test)]
mod union_args_tests;
