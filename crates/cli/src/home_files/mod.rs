//! The module that reads or writes tm's files under HOME: the savings switch and log, and the telemetry file.

mod append_savings_log;
mod append_telemetry_line;
mod append_under_home;
mod parse_user_config;
mod same_file;
mod savings_line_on;
mod savings_log_line;
mod telemetry_line;
mod telemetry_run;
mod utc_rfc3339;

#[cfg(test)]
mod integration;
#[cfg(test)]
mod parse_user_config_tests;
#[cfg(test)]
mod savings_line_on_tests;
#[cfg(test)]
mod savings_log_line_tests;
#[cfg(test)]
mod telemetry_line_tests;
#[cfg(test)]
mod utc_rfc3339_tests;

pub use append_savings_log::append_savings_log;
pub(crate) use append_telemetry_line::append_telemetry_line;
pub(crate) use append_under_home::append_under_home;
pub(crate) use parse_user_config::parse_user_config;
pub(crate) use same_file::same_file;
pub use savings_line_on::savings_line_on;
pub use savings_log_line::savings_log_line;
pub(crate) use telemetry_line::telemetry_line;
pub(crate) use telemetry_run::TelemetryRun;
pub(crate) use utc_rfc3339::utc_rfc3339;
