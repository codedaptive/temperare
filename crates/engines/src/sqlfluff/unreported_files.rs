//! `unreported_files`: the files sqlfluff was handed and skipped without a word in its report.

use procio::Output;
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// One file of sqlfluff's JSON array; only its path is read.
#[derive(Deserialize)]
struct Reported {
    filepath: String,
}

/// When `out`'s stdout is sqlfluff's JSON array and some of `files` (as they were passed, relative to the run's cwd)
/// has no entry in it: a why naming them, in `files` order, and quoting sqlfluff's stderr, which says why it skipped a
/// file over `large_file_skip_byte_limit` and says nothing for a name outside its config's `sql_file_exts`. Such a file
/// was not checked, so the rule cannot read clean. None when every file is reported, or when stdout is not the array:
/// the reader reports that.
pub(crate) fn unreported_files(out: &Output, files: &[PathBuf]) -> Option<String> {
    let reported: Vec<Reported> = serde_json::from_slice(&out.stdout).ok()?;
    let missing: Vec<String> = files
        .iter()
        .filter(|file| !reported.iter().any(|entry| Path::new(&entry.filepath) == file.as_path()))
        .map(|file| file.display().to_string())
        .collect();
    if missing.is_empty() {
        return None;
    }
    Some(format!(
        "sqlfluff did not check {}; it said: {}",
        missing.join(", "),
        String::from_utf8_lossy(&out.stderr).trim()
    ))
}
