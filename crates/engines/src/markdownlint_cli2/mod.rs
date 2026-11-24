//! The markdownlint adapter (markdownlint-cli2): its entry (`MARKDOWNLINT`) and the flow it runs for one rule, the
//! refusal of a check that is not a rule name, its no-file outcome before availability, the version from `--help`,
//! temperare's list of markdownlint rules, the options module tm hands it, the reader of its JSON output, the
//! options-file refusal and the reading of the repository's configs, and which files are Markdown files
//! (`is_markdown_source`).

mod applying_configs;
mod check_off_why;
mod config_stop;
mod is_markdown_source;
mod json_settings;
mod markdownlint_entry;
mod markdownlint_refusal;
mod markdownlint_version;
mod md_rule;
mod md_rule_of;
mod md_rules;
mod md_rules_from;
mod options_file_why;
mod parse_markdownlint;
mod run_markdownlint;
mod run_markdownlint_with;
mod tm_options;
mod yaml_settings;

#[cfg(test)]
mod applying_configs_tests;
#[cfg(test)]
mod check_off_why_tests;
#[cfg(test)]
mod config_stop_tests;
#[cfg(test)]
mod is_markdown_source_tests;
#[cfg(test)]
mod json_settings_tests;
#[cfg(test)]
mod markdownlint_entry_tests;
#[cfg(test)]
mod markdownlint_refusal_tests;
#[cfg(test)]
mod markdownlint_version_tests;
#[cfg(test)]
mod md_rule_of_tests;
#[cfg(test)]
mod md_rule_tests;
#[cfg(test)]
mod md_rules_from_tests;
#[cfg(test)]
mod md_rules_tests;
#[cfg(test)]
mod options_file_why_tests;
#[cfg(test)]
mod parse_markdownlint_tests;
#[cfg(test)]
mod run_markdownlint_with_tests;
#[cfg(test)]
mod tm_options_tests;
#[cfg(test)]
mod yaml_settings_tests;

pub(crate) use applying_configs::applying_configs;
pub(crate) use check_off_why::check_off_why;
pub(crate) use config_stop::config_stop;
pub(crate) use is_markdown_source::is_markdown_source;
pub(crate) use json_settings::json_settings;
pub use markdownlint_entry::MARKDOWNLINT;
pub(crate) use markdownlint_refusal::markdownlint_refusal;
pub(crate) use markdownlint_version::markdownlint_version;
pub(crate) use md_rule::MdRule;
pub(crate) use md_rule_of::md_rule_of;
pub(crate) use md_rules::MD_RULES;
pub(crate) use md_rules_from::MD_RULES_FROM;
pub(crate) use options_file_why::options_file_why;
pub use parse_markdownlint::parse_markdownlint;
pub(crate) use run_markdownlint::run_markdownlint;
pub(crate) use run_markdownlint_with::run_markdownlint_with;
pub(crate) use tm_options::tm_options;
pub(crate) use yaml_settings::yaml_settings;
