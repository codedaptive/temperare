//! `build_report`: the report of a check, new findings only when there is a base commit.

use crate::{build_report_with_no_files, Report, ReportError};
use procio::Oid;
use rules::{RuleFile, RuleOutcome};
use std::path::Path;

/// `build_report_with_no_files` with no flagged rule: the report of `outcomes` under `rules`, new findings only when
/// there is a base `(repository, resolved base id)`, dated today's UTC day; its errors are that entry's (see its doc).
pub fn build_report(
    rules: &RuleFile,
    outcomes: Vec<(String, RuleOutcome)>,
    base: Option<(&Path, &Oid)>,
) -> Result<Report, ReportError> {
    build_report_with_no_files(rules, outcomes, base, Vec::new())
}
