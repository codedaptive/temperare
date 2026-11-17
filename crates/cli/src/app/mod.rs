//! tm's run: the command line to an exit code, `tm rules` from finding the rule file to the listing, and `tm check`
//! from finding the rule file to the report.

mod check_world;
mod check_world_real;
mod discover_rule_file;
mod only_paths;
mod resolve_base;
mod rules_world;
mod rules_world_real;
mod run;
mod run_check;
mod run_rules;
mod select_rules;
mod write_failure;
mod write_problems;
mod write_usage_error;

#[cfg(test)]
mod check_world_real_tests;
#[cfg(test)]
mod check_world_tests;
#[cfg(test)]
mod discover_rule_file_tests;
#[cfg(test)]
mod only_paths_tests;
#[cfg(test)]
mod resolve_base_tests;
#[cfg(test)]
mod rules_world_real_tests;
#[cfg(test)]
mod rules_world_tests;
#[cfg(test)]
mod run_check_tests;
#[cfg(test)]
mod run_rules_tests;
#[cfg(test)]
mod run_tests;
#[cfg(test)]
mod select_rules_tests;
#[cfg(test)]
mod write_failure_tests;
#[cfg(test)]
mod write_problems_tests;
#[cfg(test)]
mod write_usage_error_tests;

pub use check_world::CheckWorld;
pub use discover_rule_file::discover_rule_file;
pub use rules_world::RulesWorld;
pub use run::run;
pub use run_check::run_check;
pub use run_rules::run_rules;
