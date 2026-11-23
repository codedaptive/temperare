//! The PMD adapter: the PMD entry (its SARIF command over a file list, the checks it refuses, PMD's words for a rule
//! or category it does not have, its version read past the banner), its flow (the `.java` files git lists under the
//! folder, its no-file outcome before availability, the file list handed over with CLASSPATH removed) and the reader
//! of its SARIF log, which takes exit 5 as files left unchecked.

mod java_files;
mod parse_pmd;
mod pmd_entry;
mod pmd_refusal;
mod pmd_version;
mod run_pmd;
mod run_pmd_with;
mod unknown_ruleset;

#[cfg(test)]
mod parse_pmd_tests;
#[cfg(test)]
mod pmd_entry_tests;
#[cfg(test)]
mod pmd_refusal_tests;
#[cfg(test)]
mod pmd_version_tests;
#[cfg(test)]
mod run_pmd_with_tests;
#[cfg(test)]
mod unknown_ruleset_tests;

pub(crate) use java_files::java_files;
pub use parse_pmd::parse_pmd;
pub use pmd_entry::PMD;
pub(crate) use pmd_refusal::pmd_refusal;
pub(crate) use pmd_version::pmd_version;
pub(crate) use run_pmd::run_pmd;
pub(crate) use run_pmd_with::run_pmd_with;
pub(crate) use unknown_ruleset::unknown_ruleset;
