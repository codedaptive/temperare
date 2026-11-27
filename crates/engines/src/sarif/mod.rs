//! The SARIF reader: one SARIF 2.1 log on stdout, read into hits. Of the log it reads `runs[].results[]`, and of each
//! result `ruleId` (else `rule.id`), `message.text` and its first location's `physicalLocation.artifactLocation.uri`
//! and `region.startLine`/`startColumn`. A checked result whose file is not under the rule folder fails the rule. An
//! entry whose engine prints SARIF sets its reader to `parse_sarif`, or to a reader built on `read_sarif`
//! (`parse_gitleaks` is).

mod parse_sarif;
mod read_sarif;
mod sarif_failure;
mod sarif_path;
mod strip_verbatim_disk;

#[cfg(test)]
mod parse_sarif_tests;
#[cfg(test)]
mod read_sarif_tests;
#[cfg(test)]
mod sarif_path_tests;
#[cfg(test)]
mod strip_verbatim_disk_tests;

pub use parse_sarif::parse_sarif;
pub(crate) use read_sarif::read_sarif;
pub(crate) use sarif_failure::SarifFailure;
