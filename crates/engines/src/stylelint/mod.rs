//! The stylelint adapter: its entry (`STYLELINT`: its command, the checks it refuses, stylelint's words for a rule its
//! config names but does not know), its flow for one rule (which files are CSS files, the no-file outcome before
//! availability, each file's config probed with `--print-config` and read for the check, fix and quiet, then one check
//! run with an empty suppressions file), and the readers of its JSON report, which it writes to stderr (the report per
//! file, why a file's result is no answer, the check's findings).

mod configs_stop;
mod file_report;
mod is_css_source;
mod parse_stylelint;
mod printed_config;
mod report_warning;
mod report_why;
mod run_stylelint;
mod run_stylelint_with;
mod stylelint_entry;
mod stylelint_refusal;
mod stylelint_report;
mod stylelint_unknown;

#[cfg(test)]
mod configs_stop_tests;
#[cfg(test)]
mod file_report_tests;
#[cfg(test)]
mod is_css_source_tests;
#[cfg(test)]
mod parse_stylelint_tests;
#[cfg(test)]
mod printed_config_tests;
#[cfg(test)]
mod report_warning_tests;
#[cfg(test)]
mod report_why_tests;
#[cfg(test)]
mod stylelint_entry_tests;
#[cfg(test)]
mod stylelint_refusal_tests;
#[cfg(test)]
mod stylelint_report_tests;
#[cfg(test)]
mod stylelint_unknown_tests;

pub(crate) use configs_stop::configs_stop;
pub(crate) use file_report::FileReport;
pub(crate) use is_css_source::is_css_source;
pub(crate) use parse_stylelint::parse_stylelint;
pub(crate) use printed_config::printed_config;
pub(crate) use report_warning::ReportWarning;
pub(crate) use report_why::report_why;
pub(crate) use run_stylelint::run_stylelint;
pub(crate) use run_stylelint_with::run_stylelint_with;
pub use stylelint_entry::STYLELINT;
pub(crate) use stylelint_refusal::stylelint_refusal;
pub(crate) use stylelint_report::stylelint_report;
pub(crate) use stylelint_unknown::stylelint_unknown;
