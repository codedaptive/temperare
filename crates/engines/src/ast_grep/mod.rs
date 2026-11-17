//! The ast-grep adapter: fills a rule YAML's params, builds the `ast-grep scan` command, runs it for a rule and reads
//! its JSON lines into hits.

mod ast_grep_cmd;
mod fill_params;
mod hits_ast_grep;
mod parse_ast_grep;
mod temp_rule;
mod temp_rule_drop;
mod temp_rule_write;

#[cfg(test)]
mod ast_grep_cmd_tests;
#[cfg(test)]
mod fill_params_tests;
#[cfg(test)]
mod hits_ast_grep_tests;
#[cfg(test)]
mod parse_ast_grep_tests;
#[cfg(test)]
mod temp_rule_drop_tests;
#[cfg(test)]
mod temp_rule_write_tests;

pub use ast_grep_cmd::ast_grep_cmd;
pub use fill_params::fill_params;
pub(crate) use hits_ast_grep::hits_ast_grep;
pub use parse_ast_grep::parse_ast_grep;
pub use temp_rule::TempRule;
