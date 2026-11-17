//! report: the report and its counts; new violations only, against a base commit. See the Temperare mission (Layout).

mod assemble_report;
mod build_report;
mod counts;
mod counts_of;
mod failed;
mod fold_untracked;
mod folder_from_top;
mod keep_added;
mod not_available;
mod render_text;
mod report;
mod report_error;
mod report_error_display;
mod report_error_source;
mod silenced;
mod split_by_exceptions;
mod stderr_tail;

#[cfg(test)]
mod assemble_report_tests;
#[cfg(test)]
mod build_report_tests;
#[cfg(test)]
mod counts_of_tests;
#[cfg(test)]
mod counts_tests;
#[cfg(test)]
mod failed_tests;
#[cfg(test)]
mod fold_untracked_tests;
#[cfg(test)]
mod folder_from_top_tests;
#[cfg(test)]
mod keep_added_tests;
#[cfg(test)]
mod not_available_tests;
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
mod silenced_tests;
#[cfg(test)]
mod split_by_exceptions_tests;
#[cfg(test)]
mod stderr_tail_tests;
#[cfg(test)]
mod test_json;

use assemble_report::assemble_report;
use fold_untracked::fold_untracked;
use folder_from_top::folder_from_top;
use stderr_tail::stderr_tail;

pub use build_report::build_report;
pub use counts::Counts;
pub use failed::Failed;
pub use keep_added::keep_added;
pub use not_available::NotAvailable;
pub use render_text::render_text;
pub use report::Report;
pub use report_error::ReportError;
pub use silenced::Silenced;
pub use split_by_exceptions::split_by_exceptions;
