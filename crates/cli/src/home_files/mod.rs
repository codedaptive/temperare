//! The only module that reads or writes the savings files under HOME: the config switch and the log.

mod append_savings_log;
mod parse_user_config;
mod savings_line_on;
mod savings_log_line;

#[cfg(test)]
mod integration;
#[cfg(test)]
mod parse_user_config_tests;
#[cfg(test)]
mod savings_log_line_tests;

pub use append_savings_log::append_savings_log;
pub(crate) use parse_user_config::parse_user_config;
pub use savings_line_on::savings_line_on;
pub use savings_log_line::savings_log_line;
