//! The ast-grep adapter: fills a rule YAML's params, builds the `ast-grep scan` command, runs it for a rule and reads
//! its JSON lines into hits.

mod ast_grep_cmd;
mod fill_params;
mod hits_ast_grep;
mod parse_ast_grep;
mod read_rule_yaml;
mod rule_yaml_path;
mod rule_yaml_text;
mod temp_rule;
mod temp_rule_drop;
mod temp_rule_write;

#[cfg(test)]
mod ast_grep_cmd_tests;
#[cfg(test)]
mod fill_params_tests;
#[cfg(test)]
mod integration;
#[cfg(test)]
mod parse_ast_grep_tests;
#[cfg(test)]
mod rule_yaml_path_tests;
#[cfg(test)]
mod temp_rule_drop_tests;
#[cfg(test)]
mod temp_rule_write_tests;

pub use ast_grep_cmd::ast_grep_cmd;
pub use fill_params::fill_params;
pub(crate) use hits_ast_grep::hits_ast_grep;
pub use parse_ast_grep::parse_ast_grep;
pub(crate) use read_rule_yaml::read_rule_yaml;
pub(crate) use rule_yaml_path::rule_yaml_path;
pub(crate) use rule_yaml_text::rule_yaml_text;
pub use temp_rule::TempRule;
