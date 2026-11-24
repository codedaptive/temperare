//! The ESLint adapter: the eslint entry (its command, the checks it refuses, ESLint's words for a rule its config
//! does not define, its version), its flow adding the check as a severity-only `--rule` and settling a run that linted
//! no file that git lists and the rule's paths cover, and the readers of its JSON output (its findings, the files it
//! linted).

mod eslint_entry;
mod eslint_refusal;
mod eslint_unknown;
mod eslint_version;
mod linted_files;
mod parse_eslint;
mod run_eslint;
mod run_eslint_with;

#[cfg(test)]
mod eslint_entry_tests;
#[cfg(test)]
mod eslint_refusal_tests;
#[cfg(test)]
mod eslint_unknown_tests;
#[cfg(test)]
mod eslint_version_tests;
#[cfg(test)]
mod linted_files_tests;
#[cfg(test)]
mod parse_eslint_tests;
#[cfg(test)]
mod run_eslint_with_tests;

pub use eslint_entry::ESLINT;
pub(crate) use eslint_refusal::eslint_refusal;
pub(crate) use eslint_unknown::eslint_unknown;
pub(crate) use eslint_version::eslint_version;
pub(crate) use linted_files::linted_files;
pub(crate) use parse_eslint::parse_eslint;
pub(crate) use run_eslint::run_eslint;
pub(crate) use run_eslint_with::run_eslint_with;
