//! `parse_stylelint`: stylelint's JSON report, read into the hits of one rule.

use super::report_why;
use super::stylelint_report;
use crate::printed_start::printed_start;
use crate::rel_to_folder::rel_to_folder;
use crate::types::{Hit, ParseError};
use procio::Output;
use std::path::Path;

/// The hits of rule `check` in `out`: per file in output order, its `warnings` whose `rule` is `check`, each at its
/// line and column with stylelint's text. The report is on stderr (`stylelint_report`); any exit code with a report is
/// an answer (stylelint exits 2 when it reports an error-severity problem). `cwd` is where stylelint ran and `folder`
/// the rule file's folder, both canonical; a warning in a file outside `folder` is not a hit. The first file in output
/// order whose result is no answer (`report_why`: ignored, rewritten, not parsed, invalid option) is a `ParseError`
/// with that why: such a file is never clean. Output with no report is a `ParseError` quoting what stylelint printed.
pub(crate) fn parse_stylelint(out: &Output, check: &str, cwd: &Path, folder: &Path) -> Result<Vec<Hit>, ParseError> {
    let files = stylelint_report(out)
        .ok_or_else(|| ParseError { why: format!("stylelint printed no JSON report: {}", printed_start(out)) })?;
    if let Some(why) = files.iter().find_map(|file| report_why(file, cwd, folder)) {
        return Err(ParseError { why });
    }
    Ok(files
        .into_iter()
        .flat_map(|file| {
            let path = rel_to_folder(cwd, folder, &file.source);
            file.warnings
                .into_iter()
                .filter(|warning| warning.rule.as_deref() == Some(check))
                .filter_map(move |warning| {
                    Some(Hit { path: path.clone()?, line: warning.line, col: warning.column, what: warning.text })
                })
        })
        .collect())
}
