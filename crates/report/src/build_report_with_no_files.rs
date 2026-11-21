//! `build_report_with_no_files`: the report of a check whose engines flagged rules that had no file to check.

use crate::{assemble_report, date_text, fold_untracked, folder_from_top, keep_added, utc_day, Report, ReportError};
use procio::Oid;
use rules::{Date, RuleFile, RuleOutcome};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// The report of `outcomes` under `rules`, its exceptions looked up with `RuleFile::exception_on` given today's UTC day
/// (read once from the clock); exception_on does not judge expiry yet and ignores the day. The report's `date` is that
/// day's "YYYY-MM-DD" text. With no base every finding is new, except that an `only_with_base` rule's outcomes are
/// dropped (see `assemble_report`), and nothing else is run or read. With a base,
/// `(repository, resolved base id)`, only findings on lines added since the base are new: lines git adds against it in
/// the work tree, untracked files whole, and every path under an untracked nested repository. Finding paths are
/// relative to the rule file's folder, so the folder is canonicalized (a relative one against the current directory)
/// and taken relative to the repository top. A failed git call is `ReportError::Git`; a folder that cannot be
/// canonicalized or lies outside the repository is `ReportError::Folder`.
///
/// `no_files` holds the rules the engines found no file for, each as (rule id, the engines' why). A flagged rule of
/// the repository's own is listed under `covers_nothing`; a flagged baseline rule (its id starts "tm-") is listed under
/// `not_applicable` with that why. A rule set aside as not applicable keeps its own reason, and with no base an
/// `only_with_base` rule keeps "reports only what a change adds; run with --base", whatever the flag. The outcomes of a
/// flagged rule of `rules.rules` are dropped, a failed one included, so the caller flags only a rule its engine settled
/// as having no file (engines tests a refusal first); a flagged rule set aside in `not_applicable_rules` keeps its own
/// reason and its outcomes; a flagged id naming no rule of the file is ignored (see `assemble_report`).
pub fn build_report_with_no_files(
    rules: &RuleFile,
    outcomes: Vec<(String, RuleOutcome)>,
    base: Option<(&Path, &Oid)>,
    no_files: Vec<(String, String)>,
) -> Result<Report, ReportError> {
    let (day, date) = today();
    let Some((repo, oid)) = base else {
        return Ok(assemble_report(rules, outcomes, None, day, date, &no_files, |findings| findings));
    };
    let top = procio::toplevel(repo).map_err(ReportError::Git)?;
    let folder_abs = std::fs::canonicalize(&rules.folder)
        .map_err(|err| ReportError::Folder { folder: rules.folder.clone(), why: err.to_string() })?;
    let folder = folder_from_top(&top, &folder_abs).ok_or_else(|| ReportError::Folder {
        folder: rules.folder.clone(),
        why: format!("not inside the repository at {}", top.display()),
    })?;
    let mut lines = procio::added_lines(repo, oid.as_str(), &[]).map_err(ReportError::Git)?;
    let under = fold_untracked(&mut lines, procio::untracked_files(repo).map_err(ReportError::Git)?);
    Ok(assemble_report(rules, outcomes, Some(oid.clone()), day, date, &no_files, |findings| {
        keep_added(findings, &lines, &under, &folder)
    }))
}

/// Today's UTC day, and its "YYYY-MM-DD" text. A clock set before the Unix epoch reads as its first second.
fn today() -> (Date, String) {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |since| since.as_secs());
    let (year, month, day) = utc_day(secs);
    let date = Date::new(year, month, day).expect("utc_day returns only real days in 1970..=9999 (see its doc)");
    (date, date_text(year, month, day))
}
