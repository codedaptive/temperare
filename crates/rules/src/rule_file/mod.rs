//! The rule file: reading and checking it, and the exceptions it grants.

mod check_globs;
mod check_keys;
mod check_version;
mod kit_union;
mod problem_at;
mod read_exceptions;
mod read_kits;
mod read_rule;
mod read_rules;
mod required_string;
#[allow(clippy::module_inception)]
mod rule_file;
mod rule_file_exception_for;
mod rule_file_exceptions_in_force;
mod rule_file_parse;
mod rule_file_read;
mod string_list;
mod string_value;
mod wrong_type;

#[cfg(test)]
mod check_globs_tests;
#[cfg(test)]
mod check_keys_tests;
#[cfg(test)]
mod check_version_tests;
#[cfg(test)]
mod kit_union_tests;
#[cfg(test)]
mod problem_at_tests;
#[cfg(test)]
mod read_exceptions_tests;
#[cfg(test)]
mod read_kits_tests;
#[cfg(test)]
mod read_rule_tests;
#[cfg(test)]
mod read_rules_tests;
#[cfg(test)]
mod required_string_tests;
#[cfg(test)]
mod string_list_tests;
#[cfg(test)]
mod string_value_tests;
#[cfg(test)]
mod wrong_type_tests;
#[cfg(test)]
mod rule_file_exception_for_tests;
#[cfg(test)]
mod rule_file_exceptions_in_force_tests;
#[cfg(test)]
mod rule_file_parse_tests;
#[cfg(test)]
mod rule_file_read_tests;
#[cfg(test)]
mod rule_file_tests;

pub use rule_file::RuleFile;
