//! The rule file: reading and checking it, the baseline sets it takes with its overrides of them, and the exceptions it grants.
//!
//! Past 150 lines on purpose: as a bridge file (small-files rule 4) it holds one `mod` line for each file in the
//! folder and one for its `integration` sub-folder (`#[cfg(test)]` on the tests), and the `pub use` lines; splitting
//! it would mean more sub-folders, churn with no gain.

mod apply_override;
mod baseline_rules;
mod baseline_set;
mod baseline_sets;
mod baseline_set_case_for;
mod baseline_set_yaml_for;
mod bool_value;
mod check_case_file_name;
mod check_duplicate_case_file;
mod check_failing_line;
mod check_failing_path;
mod check_globs;
mod check_keys;
mod check_not_option;
mod check_override_params;
mod check_param_types;
mod check_security_owner;
mod check_set_carried;
mod check_set_yaml;
mod check_version;
mod date_value;
mod embedded_sets;
mod engine_value;
mod house_1;
mod kind_value;
mod kit;
mod kit_holds;
mod kit_union;
mod load_set;
mod missing_param_names;
mod missing_params;
mod override_entry;
mod param_type;
mod problem_at;
mod read_baseline;
mod read_case;
mod read_case_params;
mod read_exceptions;
mod read_kits;
mod read_overrides;
mod read_required_params;
mod read_rule;
mod read_rules;
mod read_set_cases;
mod required_string;
mod resolve_baseline;
mod resolved_baseline;
#[allow(clippy::module_inception)]
mod rule_file;
mod rule_file_exception_for;
mod rule_file_exception_on;
mod rule_file_exceptions_expired;
mod rule_file_new;
mod rule_file_owner_for;
mod rule_file_parse;
mod rule_file_read;
mod rule_file_rules_in_order;
mod rule_file_rules_of_kind;
mod scope_or_problem;
mod severity_value;
mod standard_1;
mod string_list;
mod string_value;
mod syntax_problem;
mod wrong_type;

#[cfg(test)]
mod apply_override_tests;
#[cfg(test)]
mod baseline_set_tests;
#[cfg(test)]
mod baseline_sets_tests;
#[cfg(test)]
mod baseline_set_case_for_tests;
#[cfg(test)]
mod baseline_set_yaml_for_tests;
#[cfg(test)]
mod bool_value_tests;
#[cfg(test)]
mod check_case_file_name_tests;
#[cfg(test)]
mod check_duplicate_case_file_tests;
#[cfg(test)]
mod check_failing_line_tests;
#[cfg(test)]
mod check_failing_path_tests;
#[cfg(test)]
mod check_globs_tests;
#[cfg(test)]
mod check_keys_tests;
#[cfg(test)]
mod check_not_option_tests;
#[cfg(test)]
mod check_param_types_tests;
#[cfg(test)]
mod check_security_owner_tests;
#[cfg(test)]
mod check_set_carried_tests;
#[cfg(test)]
mod check_set_yaml_tests;
#[cfg(test)]
mod check_version_tests;
#[cfg(test)]
mod date_value_tests;
#[cfg(test)]
mod embedded_sets_tests;
#[cfg(test)]
mod engine_value_tests;
#[cfg(test)]
mod house_1_tests;
#[cfg(test)]
mod integration;
#[cfg(test)]
mod kind_value_tests;
#[cfg(test)]
mod kit_holds_tests;
#[cfg(test)]
mod kit_tests;
#[cfg(test)]
mod kit_union_tests;
#[cfg(test)]
mod load_set_tests;
#[cfg(test)]
mod missing_param_names_tests;
#[cfg(test)]
mod missing_params_tests;
#[cfg(test)]
mod override_entry_tests;
#[cfg(test)]
mod problem_at_tests;
#[cfg(test)]
mod read_baseline_tests;
#[cfg(test)]
mod read_case_params_tests;
#[cfg(test)]
mod read_case_tests;
#[cfg(test)]
mod read_exceptions_tests;
#[cfg(test)]
mod read_kits_tests;
#[cfg(test)]
mod read_overrides_tests;
#[cfg(test)]
mod read_required_params_tests;
#[cfg(test)]
mod read_rule_tests;
#[cfg(test)]
mod read_rules_tests;
#[cfg(test)]
mod read_set_cases_tests;
#[cfg(test)]
mod required_string_tests;
#[cfg(test)]
mod resolve_baseline_tests;
#[cfg(test)]
mod resolved_baseline_tests;
#[cfg(test)]
mod scope_or_problem_tests;
#[cfg(test)]
mod severity_value_tests;
#[cfg(test)]
mod standard_1_tests;
#[cfg(test)]
mod string_list_tests;
#[cfg(test)]
mod string_value_tests;
#[cfg(test)]
mod syntax_problem_tests;
#[cfg(test)]
mod wrong_type_tests;
#[cfg(test)]
mod rule_file_exception_for_tests;
#[cfg(test)]
mod rule_file_exception_on_tests;
#[cfg(test)]
mod rule_file_exceptions_expired_tests;
#[cfg(test)]
mod rule_file_new_tests;
#[cfg(test)]
mod rule_file_owner_for_tests;
#[cfg(test)]
mod rule_file_parse_cases_tests;
#[cfg(test)]
mod rule_file_parse_order_tests;
#[cfg(test)]
mod rule_file_parse_tests;
#[cfg(test)]
mod rule_file_parse_yaml_tests;
#[cfg(test)]
mod rule_file_read_tests;
#[cfg(test)]
mod rule_file_rules_in_order_tests;
#[cfg(test)]
mod rule_file_rules_of_kind_tests;
#[cfg(test)]
mod rule_file_tests;

pub use baseline_rules::baseline_rules;
pub use baseline_sets::baseline_sets;
pub use kit::Kit;
pub use missing_param_names::missing_param_names;
pub use rule_file::RuleFile;
