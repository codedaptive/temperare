//! tm's output: the problem line, the next-step text, the `tm rules` listing as text and as JSON (its
//! exceptions split into in force and expired), what `tm rules --loosened` found as text and as JSON, and tm check's
//! exit code, `--json` object, failed rules' stderr tails and savings estimate (its line and its `saved` JSON), the
//! `--version` line and the version and commit it shares with the telemetry line, and tm baseline test's results
//! (each case's verdict and each rule's result), exit code, next-step text, text and `--json` object.

mod baseline_exit;
mod baseline_json;
mod baseline_next;
mod baseline_rule_result;
mod baseline_rule_result_verdict;
mod baseline_text;
mod case_verdict;
mod check_json;
mod exit_code;
mod failed_tails;
mod loosened_json;
mod loosened_text;
mod next_step;
mod problem_line;
mod problems_json;
mod rule_count;
mod rules_json;
mod rules_text;
mod saved_json;
mod savings;
mod savings_from;
mod serde_name;
mod split_exceptions;
mod tokens_saved_line;
mod version_after_name;
mod version_text;

#[cfg(test)]
mod baseline_exit_tests;
#[cfg(test)]
mod baseline_json_tests;
#[cfg(test)]
mod baseline_next_tests;
#[cfg(test)]
mod baseline_rule_result_verdict_tests;
#[cfg(test)]
mod baseline_text_tests;
#[cfg(test)]
mod check_json_tests;
#[cfg(test)]
mod exit_code_tests;
#[cfg(test)]
mod failed_tails_tests;
#[cfg(test)]
mod loosened_json_tests;
#[cfg(test)]
mod loosened_text_tests;
#[cfg(test)]
mod next_step_tests;
#[cfg(test)]
mod problem_line_tests;
#[cfg(test)]
mod problems_json_tests;
#[cfg(test)]
mod rules_json_tests;
#[cfg(test)]
mod rules_text_tests;
#[cfg(test)]
mod saved_json_tests;
#[cfg(test)]
mod savings_from_tests;
#[cfg(test)]
mod serde_name_tests;
#[cfg(test)]
mod split_exceptions_tests;
#[cfg(test)]
mod tokens_saved_line_tests;
#[cfg(test)]
mod version_after_name_tests;
#[cfg(test)]
mod version_text_tests;

pub use check_json::check_json;
pub use exit_code::exit_code;
pub use failed_tails::failed_tails;
pub use loosened_json::loosened_json;
pub use loosened_text::loosened_text;
pub use next_step::next_step;
pub use problem_line::problem_line;
pub use problems_json::problems_json;
pub use rule_count::RuleCount;
pub use rules_json::rules_json;
pub use rules_text::rules_text;
pub use saved_json::saved_json;
pub use savings::Savings;
pub use savings_from::savings_from;
pub use tokens_saved_line::tokens_saved_line;
pub use version_text::version_text;

pub(crate) use baseline_exit::baseline_exit;
pub(crate) use baseline_json::baseline_json;
pub(crate) use baseline_next::baseline_next;
pub(crate) use baseline_rule_result::BaselineRuleResult;
pub(crate) use baseline_text::baseline_text;
pub(crate) use case_verdict::CaseVerdict;
pub(crate) use serde_name::serde_name;
pub(crate) use version_after_name::version_after_name;
