//! The ast-grep adapter: the ast-grep entry (its `ast-grep scan` command), the flow that takes a rule YAML (the set's
//! embedded text, or a file inside the rule file's folder) and fills its params, and the reader of its JSON lines; and
//! its count mode (`count = "setup-vs-tests"`), which counts each test file's setup lines against its test lines and
//! reports the files a change against the base adds lines to whose setup is heavier.

mod added_in_folder;
mod ast_grep_entry;
mod ast_grep_parse_error;
mod count_ast_grep;
mod count_file;
mod count_match;
mod count_mode;
mod file_count;
mod fill_params;
mod hits_ast_grep;
mod open_in_folder;
mod parse_ast_grep;
mod parse_count_matches;
mod read_rule_yaml;
mod rule_yaml_path;
mod rule_yaml_text;
mod setup_heavier_hits;
mod split_count_params;

#[cfg(test)]
mod ast_grep_entry_tests;
#[cfg(test)]
mod ast_grep_parse_error_tests;
#[cfg(test)]
mod count_file_tests;
#[cfg(test)]
mod fill_params_tests;
#[cfg(test)]
mod integration;
#[cfg(test)]
mod parse_ast_grep_tests;
#[cfg(test)]
mod parse_count_matches_tests;
#[cfg(test)]
mod setup_heavier_hits_tests;
#[cfg(test)]
mod split_count_params_tests;

pub(crate) use added_in_folder::added_in_folder;
pub use ast_grep_entry::AST_GREP;
pub(crate) use ast_grep_parse_error::ast_grep_parse_error;
pub(crate) use count_ast_grep::count_ast_grep;
pub(crate) use count_file::count_file;
pub(crate) use count_match::CountMatch;
pub(crate) use count_mode::CountMode;
pub(crate) use file_count::FileCount;
pub use fill_params::fill_params;
pub(crate) use hits_ast_grep::hits_ast_grep;
pub(crate) use open_in_folder::open_in_folder;
pub use parse_ast_grep::parse_ast_grep;
pub(crate) use parse_count_matches::parse_count_matches;
pub(crate) use read_rule_yaml::read_rule_yaml;
pub(crate) use rule_yaml_path::rule_yaml_path;
pub(crate) use rule_yaml_text::rule_yaml_text;
pub(crate) use setup_heavier_hits::setup_heavier_hits;
pub(crate) use split_count_params::split_count_params;
