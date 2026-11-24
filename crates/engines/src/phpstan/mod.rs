//! The PHPStan adapter: its entry (`PHPSTAN`) and the flow it runs for one rule, the refusal of a check that is not a
//! level, its no-file outcome before availability, PHPStan's unknown-level and no-files words, the reader of its JSON
//! output, the repository config it includes and the wrapper config around it, and which files are PHP files
//! (`is_php_source`).

mod is_php_source;
mod no_php_analysed;
mod parse_phpstan;
mod phpstan_config;
mod phpstan_entry;
mod phpstan_refusal;
mod repo_config;
mod run_phpstan;
mod unknown_level;

#[cfg(test)]
mod is_php_source_tests;
#[cfg(test)]
mod no_php_analysed_tests;
#[cfg(test)]
mod parse_phpstan_tests;
#[cfg(test)]
mod phpstan_config_tests;
#[cfg(test)]
mod phpstan_entry_tests;
#[cfg(test)]
mod phpstan_refusal_tests;
#[cfg(test)]
mod repo_config_tests;
#[cfg(test)]
mod unknown_level_tests;

pub(crate) use is_php_source::is_php_source;
pub(crate) use no_php_analysed::no_php_analysed;
pub use parse_phpstan::parse_phpstan;
pub(crate) use phpstan_config::phpstan_config;
pub use phpstan_entry::PHPSTAN;
pub(crate) use phpstan_refusal::phpstan_refusal;
pub(crate) use repo_config::repo_config;
pub(crate) use run_phpstan::run_phpstan;
pub(crate) use unknown_level::unknown_level;
