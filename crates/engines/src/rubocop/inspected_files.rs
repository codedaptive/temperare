//! `inspected_files`: the files a rubocop check run says it inspected.

use rules::RelPath;
use serde::Deserialize;

/// rubocop's `--format json` report, only as far as its file list.
#[derive(Deserialize)]
struct Report {
    files: Vec<File>,
}

/// One inspected file; its offenses are not read here.
#[derive(Deserialize)]
struct File {
    path: String,
}

/// The `"files"[].path` values of rubocop's JSON report in `stdout`, in order, each as a `RelPath`. rubocop lists
/// every file it inspected, a clean one with no offenses, each relative to the folder it ran in. Empty when `stdout`
/// is not that report: `parse_rubocop` has already refused such output on the check run.
pub(crate) fn inspected_files(stdout: &[u8]) -> Vec<RelPath> {
    serde_json::from_slice::<Report>(stdout)
        .map(|report| report.files.iter().map(|file| RelPath::new(&file.path)).collect())
        .unwrap_or_default()
}
