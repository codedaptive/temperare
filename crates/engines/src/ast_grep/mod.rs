//! The ast-grep adapter: the ast-grep entry (its `ast-grep scan` command), the flow that reads a rule YAML inside
//! the rule file's folder and fills its params, and the reader of its JSON lines.

mod ast_grep_entry;
mod fill_params;
mod hits_ast_grep;
mod open_in_folder;
mod parse_ast_grep;
mod read_rule_yaml;
mod rule_yaml_path;
mod rule_yaml_text;

#[cfg(test)]
mod ast_grep_entry_tests;
#[cfg(test)]
mod fill_params_tests;
#[cfg(test)]
mod integration;
#[cfg(test)]
mod parse_ast_grep_tests;
#[cfg(test)]
mod rule_yaml_path_tests;

pub use ast_grep_entry::AST_GREP;
pub use fill_params::fill_params;
pub(crate) use hits_ast_grep::hits_ast_grep;
pub(crate) use open_in_folder::open_in_folder;
pub use parse_ast_grep::parse_ast_grep;
pub(crate) use read_rule_yaml::read_rule_yaml;
pub(crate) use rule_yaml_path::rule_yaml_path;
pub(crate) use rule_yaml_text::rule_yaml_text;
