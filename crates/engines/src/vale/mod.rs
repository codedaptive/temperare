//! The Vale adapter: its entry (`VALE`) and the flow it runs for one rule, the refusal of a check that is not
//! `Style.Rule`, the filter that makes Vale name an unknown rule, Vale's error objects and its unknown-rule words, the
//! reader of its JSON output, the repository config read from `vale ls-config` and the stops it gives, the wrapper
//! config, and which files Vale is given (`is_vale_source`).
//!
//! The design (S-72, S-81):
//! - **A wrapper config.** tm never runs Vale on the repository's config (its `.vale.ini`, or the file the rule's `config` names) directly. It reads that config through
//!   `vale ls-config` (`ValeConfig::read`), stops when it is missing, outside the repository or does not name the
//!   check's style (`config_stop`), and writes its own ini in a temporary folder (`vale_wrapper`), removed after the
//!   run. The wrapper copies only the repository's `StylesPath` and `Vocab`, and turns on the check's style for every
//!   file passed.
//! - **The files tm passes.** Vale is given the rule's git-listed files under its paths, in the formats Vale reads
//!   (`is_vale_source`), so the rule file decides the scope and an empty answer means clean. With none the rule is
//!   `NoFiles` before availability (`engine_listed`).
//! - **Vocab is honoured, a stated limit** (S-81 (2)). A vocabulary defines which words are correct everywhere in the
//!   repository, as a rule's own options do; it lets no finding through at one place. So `Vocab` is copied, and
//!   `Vale.Spelling` accepts its words (tests/vale.rs, the `vocab-on` row).
//! - **Inline comments stay on** (S-67 (2)). Vale cannot be told to ignore `<!-- vale off -->` or
//!   `<!-- vale Style.Rule = NO -->` in the checked files, and tm parses no file itself, so these still hide findings:
//!   a gap the baseline's suppression rules are for.
//! - **The config's skip keys are off** (S-67 (1)). `BlockIgnores`, `TokenIgnores`, `IgnoredScopes`,
//!   `SkippedScopes`, `IgnoredClasses` and `Style.Rule = NO`, global or in a section, are never copied into the
//!   wrapper, so Vale never reads them and nothing is refused for them.

mod config_stop;
mod is_vale_source;
mod parse_vale;
mod run_vale;
mod vale_config;
mod vale_config_read;
mod vale_entry;
mod vale_errors;
mod vale_filter;
mod vale_refusal;
mod vale_unknown;
mod vale_wrapper;

#[cfg(test)]
mod config_stop_tests;
#[cfg(test)]
mod is_vale_source_tests;
#[cfg(test)]
mod parse_vale_tests;
#[cfg(test)]
mod vale_config_read_tests;
#[cfg(test)]
mod vale_entry_tests;
#[cfg(test)]
mod vale_errors_tests;
#[cfg(test)]
mod vale_filter_tests;
#[cfg(test)]
mod vale_refusal_tests;
#[cfg(test)]
mod vale_unknown_tests;
#[cfg(test)]
mod vale_wrapper_tests;

pub(crate) use config_stop::config_stop;
pub(crate) use is_vale_source::is_vale_source;
pub use parse_vale::parse_vale;
pub(crate) use run_vale::run_vale;
pub(crate) use vale_config::ValeConfig;
pub use vale_entry::VALE;
pub(crate) use vale_errors::vale_errors;
pub(crate) use vale_filter::vale_filter;
pub(crate) use vale_refusal::vale_refusal;
pub(crate) use vale_unknown::vale_unknown;
pub(crate) use vale_wrapper::vale_wrapper;
