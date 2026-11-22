//! The gitleaks adapter: the gitleaks entry (its redacted SARIF command over the folder, the checks it refuses,
//! gitleaks' words for a rule id it does not know), its flow (the rule's config kept, stderr cut to error lines) and
//! the reader of its SARIF log, which never copies gitleaks' message into a hit.

mod error_lines;
mod gitleaks_entry;
mod gitleaks_refusal;
mod parse_gitleaks;
mod run_gitleaks;
mod unknown_rule_id;

#[cfg(test)]
mod error_lines_tests;
#[cfg(test)]
mod gitleaks_entry_tests;
#[cfg(test)]
mod gitleaks_refusal_tests;
#[cfg(test)]
mod parse_gitleaks_tests;
#[cfg(test)]
mod run_gitleaks_tests;
#[cfg(test)]
mod unknown_rule_id_tests;

pub(crate) use error_lines::error_lines;
pub use gitleaks_entry::GITLEAKS;
pub(crate) use gitleaks_refusal::gitleaks_refusal;
pub use parse_gitleaks::parse_gitleaks;
pub(crate) use run_gitleaks::run_gitleaks;
pub(crate) use unknown_rule_id::unknown_rule_id;
