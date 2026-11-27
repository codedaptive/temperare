//! `build_report_with_no_files`: the report of a check whose engines flagged rules that had no file to check.

use crate::{assemble_report, fold_untracked, folder_from_top, keep_added, utc_date, Report, ReportError};
use ccvcslib::{AddedLines, GitVcs, Revision, Vcs};
use crate::Oid;
use rules::{Date, RuleFile, RuleOutcome};
use std::collections::BTreeMap;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// The report of `outcomes` under `rules`, its exceptions looked up with `RuleFile::exception_on` given today's UTC day
/// (read once from the clock); an exception past its `expires` day silences nothing. The report's `date` is that day's
/// "YYYY-MM-DD" text; its `expired` lists the exceptions past their `expires` day on that same day, and
/// `exceptions_in_force` counts the others. With no base every finding is new, except that an `only_with_base` rule's
/// outcomes are dropped (see `out_of_play`), and nothing else is run or read. With a base,
/// `(repository, resolved base id)`, only findings on lines added since the base are new: lines git adds against it in
/// the work tree, untracked files whole, and every path under an untracked nested repository. The added lines and the
/// untracked entries are asked of ccvcslib at the repository top as git gave it, so a subfolder `repository` still
/// covers the whole working copy. Finding paths are relative to the rule file's folder, so the folder (a relative one
/// against the current directory) and the repository top are both canonicalized, giving one path form to compare
/// (git prints `C:/...` on Windows where canonicalize gives `\\?\C:\...`), and the folder is taken relative to the
/// top. A failed ccvcslib question is `ReportError::Vcs`; a folder or top that cannot be canonicalized, or a folder
/// outside the repository, is `ReportError::Folder`.
///
/// `no_files` holds the rules the engines found no file for, each as (rule id, the engines' why). A flagged rule of
/// the repository's own is listed under `covers_nothing`; a flagged baseline rule (its id starts "tm-") is listed under
/// `not_applicable` with that why. A rule set aside as not applicable keeps its own reason, and with no base an
/// `only_with_base` rule keeps "reports only what a change adds; run with --base", whatever the flag. The outcomes of a
/// flagged rule of `rules.rules` are dropped, a failed one included, so the caller flags only a rule its engine settled
/// as having no file (engines tests a refusal first); a flagged rule set aside in `not_applicable_rules` keeps its own
/// reason and its outcomes; a flagged id naming no rule of the file is ignored (see `out_of_play`).
///
/// The report's `record` holds the telemetry record's rules (see `rules_record`), from the same outcomes; with a base,
/// its findings are cut by the same added-lines map asked of ccvcslib once.
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
    let vcs = GitVcs::default();
    let top = vcs.working_copy_root(repo).map_err(ReportError::Vcs)?;
    let folder_abs = std::fs::canonicalize(&rules.folder)
        .map_err(|err| ReportError::Folder { folder: rules.folder.clone(), why: err.to_string() })?;
    let top_abs = std::fs::canonicalize(&top).map_err(|err| ReportError::Folder {
        folder: rules.folder.clone(),
        why: format!("cannot resolve the repository top {}: {err}", top.display()),
    })?;
    let folder = folder_from_top(&top_abs, &folder_abs).ok_or_else(|| ReportError::Folder {
        folder: rules.folder.clone(),
        why: format!("not inside the repository at {}", top.display()),
    })?;
    let revision = Revision(oid.as_str().to_string());
    let mut lines = by_path(vcs.added_lines(&top, &revision, &[]).map_err(ReportError::Vcs)?);
    let under = fold_untracked(&mut lines, vcs.untracked_entries(&top).map_err(ReportError::Vcs)?);
    Ok(assemble_report(rules, outcomes, Some(oid.clone()), day, date, &no_files, |findings| {
        keep_added(findings, &lines, &under, &folder)
    }))
}

/// ccvcslib's added lines keyed by path, in the shape `keep_added` reads: each run `start..end` as it is (both
/// 1-based, end exclusive), and a file that only lost lines kept with no ranges.
fn by_path(added: Vec<AddedLines>) -> BTreeMap<PathBuf, Vec<Range<u32>>> {
    added
        .into_iter()
        .map(|file| (file.path, file.lines.into_iter().map(|run| run.start..run.end).collect()))
        .collect()
}

/// Today's UTC day, and its "YYYY-MM-DD" text. A clock set before the Unix epoch reads as its first second.
fn today() -> (Date, String) {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |since| since.as_secs());
    let date = utc_date(secs);
    (date, date.to_string())
}
