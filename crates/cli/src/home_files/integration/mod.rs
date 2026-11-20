//! The home_files tests that touch a disk HOME the test made; run by GROUP=integration.

mod append_savings_log_tests;
#[cfg(unix)]
mod same_file_tests;
mod savings_line_on_tests;
