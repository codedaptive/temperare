//! The SARIF reader: one SARIF 2.1 log on stdout, read into hits. Of the log it reads `runs[].results[]`, and of each
//! result `ruleId` (else `rule.id`), `message.text` and its first location's `physicalLocation.artifactLocation.uri`
//! and `region.startLine`/`startColumn`. No engine adapter prints SARIF yet: an entry whose engine prints SARIF sets
//! its reader to `parse_sarif`.

mod parse_sarif;
mod sarif_path;

#[cfg(test)]
mod parse_sarif_tests;
#[cfg(test)]
mod sarif_path_tests;

pub use parse_sarif::parse_sarif;
