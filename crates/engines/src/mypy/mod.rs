//! The mypy adapter: its entry (`MYPY`) and the flow it runs for one rule, the refusal of a check that is not one
//! error code, its no-file outcome before availability, mypy's unknown-code words, the reader of its JSON-lines
//! output, its command, and the listing of the Python files in a repository (tracked, and untracked but not ignored;
//! never a symlink). Its config reading: which config file mypy itself would read from a folder, the settings in an
//! INI config (mypy.ini, .mypy.ini, setup.cfg) or a pyproject.toml that can silence findings, the truth of a flag's
//! text, and whether a config silences a check's findings outside the rule file.

mod config_suppression;
mod config_true;
mod ini_settings;
mod mypy_cmd;
mod mypy_config;
mod mypy_entry;
mod mypy_refusal;
mod mypy_setting;
mod parse_mypy;
mod python_files;
mod run_mypy;
mod toml_settings;
mod unknown_code;

#[cfg(test)]
mod config_suppression_tests;
#[cfg(test)]
mod config_true_tests;
#[cfg(test)]
mod ini_settings_tests;
#[cfg(test)]
mod mypy_cmd_tests;
#[cfg(test)]
mod mypy_config_tests;
#[cfg(test)]
mod mypy_entry_tests;
#[cfg(test)]
mod mypy_refusal_tests;
#[cfg(test)]
mod parse_mypy_tests;
#[cfg(test)]
mod toml_settings_tests;
#[cfg(test)]
mod unknown_code_tests;

pub(crate) use config_suppression::config_suppression;
pub(crate) use config_true::config_true;
pub(crate) use ini_settings::ini_settings;
pub(crate) use mypy_cmd::mypy_cmd;
pub(crate) use mypy_config::mypy_config;
pub use mypy_entry::MYPY;
pub(crate) use mypy_refusal::mypy_refusal;
pub(crate) use mypy_setting::MypySetting;
pub use parse_mypy::parse_mypy;
pub(crate) use python_files::python_files;
pub(crate) use run_mypy::run_mypy;
pub(crate) use toml_settings::toml_settings;
pub(crate) use unknown_code::unknown_code;
