//! The sqlfluff adapter: its entry (`SQLFLUFF`) and the flow it runs for one rule, the refusal of a check that is not
//! one rule reference, its no-file outcome before availability, sqlfluff's unknown-rule and no-dialect words, the files
//! it was handed and left out of its report, the reader of its JSON report, its command, and which files are SQL.

mod is_sql_source;
mod no_dialect;
mod parse_sqlfluff;
mod run_sqlfluff;
mod sqlfluff_cmd;
mod sqlfluff_entry;
mod sqlfluff_refusal;
mod unknown_rule;
mod unreported_files;

#[cfg(test)]
mod is_sql_source_tests;
#[cfg(test)]
mod no_dialect_tests;
#[cfg(test)]
mod parse_sqlfluff_tests;
#[cfg(test)]
mod sqlfluff_cmd_tests;
#[cfg(test)]
mod sqlfluff_entry_tests;
#[cfg(test)]
mod sqlfluff_refusal_tests;
#[cfg(test)]
mod unknown_rule_tests;
#[cfg(test)]
mod unreported_files_tests;

pub(crate) use is_sql_source::is_sql_source;
pub(crate) use no_dialect::no_dialect;
pub use parse_sqlfluff::parse_sqlfluff;
pub(crate) use run_sqlfluff::run_sqlfluff;
pub(crate) use sqlfluff_cmd::sqlfluff_cmd;
pub use sqlfluff_entry::SQLFLUFF;
pub(crate) use sqlfluff_refusal::sqlfluff_refusal;
pub(crate) use unknown_rule::unknown_rule;
pub(crate) use unreported_files::unreported_files;
