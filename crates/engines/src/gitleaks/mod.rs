//! The gitleaks adapter: the gitleaks entry (its redacted SARIF command over the folder, the checks it refuses,
//! gitleaks' words for a rule id it does not know), its flow (a guard that fails the rule when a gitleaks allowlist or
//! `.gitleaksignore` would silence findings outside the rule file, or a config does not parse, the config read with
//! the `toml` crate and its keys matched in any ASCII case as gitleaks reads them; the rule's config kept, stderr cut
//! to error lines) and the reader of its SARIF log, which never copies gitleaks' message into a hit.

mod allowlist_key;
mod config_why;
mod error_lines;
mod gitleaks_entry;
mod gitleaks_refusal;
mod ignore_listed;
mod parse_gitleaks;
mod run_gitleaks;
mod suppression_why;
mod unknown_rule_id;

#[cfg(test)]
mod allowlist_key_tests;
#[cfg(test)]
mod config_why_tests;
#[cfg(test)]
mod error_lines_tests;
#[cfg(test)]
mod gitleaks_entry_tests;
#[cfg(test)]
mod gitleaks_refusal_tests;
#[cfg(test)]
mod ignore_listed_tests;
#[cfg(test)]
mod parse_gitleaks_tests;
#[cfg(test)]
mod run_gitleaks_tests;
#[cfg(test)]
mod unknown_rule_id_tests;

pub(crate) use allowlist_key::allowlist_key;
pub(crate) use config_why::config_why;
pub(crate) use error_lines::error_lines;
pub use gitleaks_entry::GITLEAKS;
pub(crate) use gitleaks_refusal::gitleaks_refusal;
pub(crate) use ignore_listed::ignore_listed;
pub use parse_gitleaks::parse_gitleaks;
pub(crate) use run_gitleaks::run_gitleaks;
pub(crate) use suppression_why::suppression_why;
pub(crate) use unknown_rule_id::unknown_rule_id;
