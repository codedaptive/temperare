//! The crate-level in-crate zone integration tests (real filesystem); the integration group runs them with
//! `--lib integration::`.

mod change_of_tests;
mod engine_hits_tests;
#[cfg(unix)]
mod eslint_local_bin_tests;
#[cfg(unix)]
mod files_in_folder_tests;
#[cfg(unix)]
mod is_regular_file_tests;
#[cfg(unix)]
mod program_for_tests;
mod rule_folder_tests;
mod ruff_unknown_selector_tests;
mod run_rules_counted_tests;
mod run_rules_counted_with_group_tests;
mod run_rules_tests;
mod run_rules_versioned_at_tests;
mod run_rules_versioned_with_tests;
mod run_stylelint_with_tests;
#[cfg(unix)]
mod source_files_tests;
