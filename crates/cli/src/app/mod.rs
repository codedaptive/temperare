//! tm's run: the command line to an exit code, `tm rules` from finding the rule file to the listing, `tm check` from
//! finding the rule file to the report, `tm baseline test` from the sets' rules to each case's verdict, and
//! `temperare install` and `uninstall` on the disk.

mod baseline_world;
mod baseline_world_real;
mod case_path;
mod case_rel_path;
mod check_world;
mod check_world_real;
mod discover_rule_file;
mod judge_case;
mod leftover_folders_next;
mod only_paths;
mod report_engines;
mod resolve_base;
mod rule_counts;
mod rules_world;
mod rules_world_real;
mod run;
mod run_baseline_test;
mod run_case;
mod run_change_case;
mod run_check;
mod run_install;
mod run_rules;
mod run_uninstall;
mod select_rules;
mod unrecorded_next;
mod write_case_files;
mod write_failure;
mod write_problems;
mod write_usage_error;

#[cfg(test)]
mod case_path_tests;
#[cfg(test)]
mod case_rel_path_tests;
#[cfg(test)]
mod check_world_tests;
#[cfg(test)]
mod discover_rule_file_tests;
#[cfg(test)]
mod judge_case_tests;
#[cfg(test)]
mod leftover_folders_next_tests;
#[cfg(test)]
mod only_paths_tests;
#[cfg(test)]
mod report_engines_tests;
#[cfg(test)]
mod resolve_base_tests;
#[cfg(test)]
mod rules_world_tests;
#[cfg(test)]
mod integration;
#[cfg(test)]
mod rule_counts_tests;
#[cfg(test)]
mod run_baseline_test_tests;
#[cfg(test)]
mod run_case_tests;
#[cfg(test)]
mod run_change_case_tests;
#[cfg(test)]
mod run_check_savings_tests;
#[cfg(test)]
mod run_check_not_available_tests;
#[cfg(test)]
mod run_check_tests;
#[cfg(test)]
mod run_install_tests;
#[cfg(test)]
mod run_rules_tests;
#[cfg(test)]
mod run_tests;
#[cfg(test)]
mod run_uninstall_tests;
#[cfg(test)]
mod select_rules_tests;
#[cfg(test)]
mod unrecorded_next_tests;
#[cfg(test)]
mod write_case_files_tests;
#[cfg(test)]
mod write_failure_tests;
#[cfg(test)]
mod write_problems_tests;
#[cfg(test)]
mod write_usage_error_tests;

pub use baseline_world::BaselineWorld;
pub use check_world::CheckWorld;
pub use discover_rule_file::discover_rule_file;
pub use rules_world::RulesWorld;
pub use run::run;
pub use run_baseline_test::run_baseline_test;
pub use run_check::run_check;
pub use run_install::run_install;
pub use run_rules::run_rules;
pub use run_uninstall::run_uninstall;
