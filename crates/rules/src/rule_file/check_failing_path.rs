//! `check_failing_path`: a failing case's finding is in one of that case's own files.

use super::problem_at::problem_at;
use crate::Problem;

/// A Problem at `path_at`, the failing `path` value in the set's `text`, when `path` is not one of `listed`, the
/// failing case's file names as `files` lists them (carried or not) for baseline rule `id`. An empty `listed` gives
/// nothing here: a case that lists no files is already its own Problem.
pub(crate) fn check_failing_path(
    text: &str,
    path: &str,
    path_at: usize,
    listed: &[String],
    id: &str,
    out: &mut Vec<Problem>,
) {
    if listed.is_empty() || listed.iter().any(|file| file == path) {
        return;
    }
    out.push(problem_at(
        text,
        path_at,
        format!("failing path \"{path}\" of baseline rule \"{id}\" is not one of its failing files"),
        "Set path to one of the failing case's files, or list it in files.",
    ));
}
