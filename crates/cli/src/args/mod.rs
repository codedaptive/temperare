//! cctao's command line: the parsed command, its arguments (cctao install's and cctao baseline test's among them), the
//! usage text and the usage error.

mod baseline_args;
mod check_args;
mod command;
mod flag_value;
mod install_args;
mod parse_args;
mod parse_baseline_args;
mod parse_check_args;
mod parse_install_args;
mod parse_rules_args;
mod rules_args;
mod usage;
mod usage_error;

#[cfg(test)]
mod check_args_tests;
#[cfg(test)]
mod command_tests;
#[cfg(test)]
mod flag_value_tests;
#[cfg(test)]
mod install_args_tests;
#[cfg(test)]
mod parse_args_tests;
#[cfg(test)]
mod parse_baseline_args_tests;
#[cfg(test)]
mod parse_check_args_tests;
#[cfg(test)]
mod parse_install_args_tests;
#[cfg(test)]
mod parse_rules_args_tests;
#[cfg(test)]
mod rules_args_tests;
#[cfg(test)]
mod usage_error_tests;
#[cfg(test)]
mod usage_tests;

pub use baseline_args::BaselineArgs;
pub use check_args::CheckArgs;
pub use command::Command;
pub use install_args::InstallArgs;
pub use parse_args::parse_args;
pub use rules_args::RulesArgs;
pub use usage::USAGE;
pub use usage_error::UsageError;
