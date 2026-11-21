//! `check_failing_line`: a failing case's finding line is a line of the file it is in.

use super::problem_at::problem_at;
use crate::{CaseFile, Problem};

/// A Problem at `line_at`, the failing `line` value in the set's `text`, when `line` is past the last line of the
/// carried case file `path` among `files` (lines counted as `str::lines` does, so an empty file has none and every
/// line is past it), for baseline rule `id`. A `path` that is not among `files` gives nothing here: it is unlisted
/// or not carried, and is already its own Problem.
pub(crate) fn check_failing_line(
    text: &str,
    path: &str,
    line: usize,
    line_at: usize,
    files: &[CaseFile],
    id: &str,
    out: &mut Vec<Problem>,
) {
    let Some(file) = files.iter().find(|file| file.path == path) else {
        return;
    };
    let lines = file.text.lines().count();
    if line <= lines {
        return;
    }
    out.push(problem_at(
        text,
        line_at,
        format!("failing line {line} of baseline rule \"{id}\" is past the end of \"{path}\", which has {lines} lines"),
        "Set line to the line of the finding in that file, or correct the case file.",
    ));
}
