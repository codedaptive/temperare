//! tm's output: the problem line, the next-step text, the `tm rules` listing as text and as JSON, and tm check's
//! exit code, `--json` object, failed rules' stderr tails and savings estimate (its line and its `saved` JSON), and
//! the `--version` line.

mod check_json;
mod exit_code;
mod failed_tails;
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
mod tokens_saved_line;
mod version_text;

#[cfg(test)]
mod check_json_tests;
#[cfg(test)]
mod exit_code_tests;
#[cfg(test)]
mod failed_tails_tests;
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
mod tokens_saved_line_tests;
#[cfg(test)]
mod version_text_tests;

pub use check_json::check_json;
pub use exit_code::exit_code;
pub use failed_tails::failed_tails;
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

pub(crate) use serde_name::serde_name;
