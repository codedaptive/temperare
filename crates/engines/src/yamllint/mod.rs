//! The yamllint adapter: its entry (`YAMLLINT`) and the flow it runs for one rule, the refusal of a check that is not
//! one rule id, which files are YAML files (`is_yaml_source`), the repository config the run extends and the `-d`
//! config text built on it, yamllint's no-such-rule words, and the reader of its parsable output.

mod config_data;
mod is_yaml_source;
mod no_such_rule;
mod parse_yamllint;
mod run_yamllint;
mod yamllint_config;
mod yamllint_entry;
mod yamllint_refusal;

#[cfg(test)]
mod config_data_tests;
#[cfg(test)]
mod is_yaml_source_tests;
#[cfg(test)]
mod no_such_rule_tests;
#[cfg(test)]
mod parse_yamllint_tests;
#[cfg(test)]
mod yamllint_config_tests;
#[cfg(test)]
mod yamllint_entry_tests;
#[cfg(test)]
mod yamllint_refusal_tests;

pub(crate) use config_data::config_data;
pub(crate) use is_yaml_source::is_yaml_source;
pub(crate) use no_such_rule::no_such_rule;
pub use parse_yamllint::parse_yamllint;
pub(crate) use run_yamllint::run_yamllint;
pub(crate) use yamllint_config::yamllint_config;
pub use yamllint_entry::YAMLLINT;
pub(crate) use yamllint_refusal::yamllint_refusal;
