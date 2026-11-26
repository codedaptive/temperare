//! report: the report and its counts; new violations only, against a base commit.

mod assemble_report;
mod build_report_with_no_files;
mod counts;
mod counts_of;
mod covers_nothing;
mod engine_version;
mod expired;
mod failed;
mod fold_untracked;
mod folder_from_top;
mod keep_added;
mod not_applicable;
mod not_available;
mod oid;
mod oid_as_str;
mod oid_display;
mod oid_parse;
mod out_of_play;
mod render_text;
mod report;
mod report_error;
mod report_error_display;
mod report_error_source;
mod rule_count;
mod set_aside_why;
mod silenced;
mod silenced_by_rule;
mod split_by_exceptions;
mod stderr_tail;
mod utc_date;
mod utc_day;

#[cfg(test)]
mod assemble_report_tests;
#[cfg(test)]
mod counts_of_tests;
#[cfg(test)]
mod counts_tests;
#[cfg(test)]
mod covers_nothing_tests;
#[cfg(test)]
mod engine_version_tests;
#[cfg(test)]
mod expired_tests;
#[cfg(test)]
mod failed_tests;
#[cfg(test)]
mod fold_untracked_tests;
#[cfg(test)]
mod folder_from_top_tests;
#[cfg(test)]
mod keep_added_tests;
#[cfg(test)]
mod not_applicable_tests;
#[cfg(test)]
mod not_available_tests;
#[cfg(test)]
mod oid_as_str_tests;
#[cfg(test)]
mod oid_display_tests;
#[cfg(test)]
mod oid_parse_tests;
#[cfg(test)]
mod oid_tests;
#[cfg(test)]
mod out_of_play_tests;
#[cfg(test)]
mod render_text_tests;
#[cfg(test)]
mod report_error_display_tests;
#[cfg(test)]
mod report_error_source_tests;
#[cfg(test)]
mod report_error_tests;
#[cfg(test)]
mod report_tests;
#[cfg(test)]
mod rule_count_tests;
#[cfg(test)]
mod set_aside_why_tests;
#[cfg(test)]
mod silenced_by_rule_tests;
#[cfg(test)]
mod silenced_tests;
#[cfg(test)]
mod split_by_exceptions_tests;
#[cfg(test)]
mod stderr_tail_tests;
#[cfg(test)]
mod test_json;
#[cfg(test)]
mod utc_date_tests;
#[cfg(test)]
mod utc_day_tests;

use assemble_report::assemble_report;
use fold_untracked::fold_untracked;
use folder_from_top::folder_from_top;
use silenced_by_rule::silenced_by_rule;
use stderr_tail::stderr_tail;
use utc_day::utc_day;

pub use build_report_with_no_files::build_report_with_no_files;
pub use counts::Counts;
pub use covers_nothing::CoversNothing;
pub use engine_version::EngineVersion;
pub use expired::Expired;
pub use failed::Failed;
pub use keep_added::keep_added;
pub use not_applicable::NotApplicable;
pub use not_available::NotAvailable;
pub use oid::Oid;
pub use out_of_play::out_of_play;
pub use render_text::render_text;
pub use report::Report;
pub use report_error::ReportError;
pub use rule_count::RuleCount;
pub use set_aside_why::set_aside_why;
pub use silenced::Silenced;
pub use split_by_exceptions::split_by_exceptions;
pub use utc_date::utc_date;
