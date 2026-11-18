//! tm's output: the problem line, the next-step text, the `tm rules` listing as text and as JSON, and tm check's
//! exit code, `--json` object, not-available engine names and failed rules' stderr tails, and the `--version` line.

mod check_json;
mod exit_code;
mod failed_tails;
mod next_step;
mod not_available_names;
mod problem_line;
mod problems_json;
mod rules_json;
mod rules_text;
mod serde_name;
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
mod not_available_names_tests;
#[cfg(test)]
mod problem_line_tests;
#[cfg(test)]
mod problems_json_tests;
#[cfg(test)]
mod rules_json_tests;
#[cfg(test)]
mod rules_text_tests;
#[cfg(test)]
mod serde_name_tests;
#[cfg(test)]
mod version_text_tests;

pub use check_json::check_json;
pub use exit_code::exit_code;
pub use failed_tails::failed_tails;
pub use next_step::next_step;
pub use not_available_names::not_available_names;
pub use problem_line::problem_line;
pub use problems_json::problems_json;
pub use rules_json::rules_json;
pub use rules_text::rules_text;
pub use version_text::version_text;
