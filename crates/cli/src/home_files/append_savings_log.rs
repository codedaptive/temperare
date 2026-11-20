//! `append_savings_log`: adds one line to the savings log under HOME.

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

/// Appends `line` and a newline, in one write, to `<home>/.local/state/temperare/savings.jsonl`, making the folder
/// when missing. Does nothing when there is no HOME; any failure is silent.
pub fn append_savings_log(home: Option<&Path>, line: &str) {
    let Some(home) = home.filter(|home| !home.as_os_str().is_empty()) else {
        return;
    };
    // The log is a convenience: a check never fails or prints because it could not be written.
    let _ = append(&home.join(".local/state/temperare"), line);
}

fn append(folder: &Path, line: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(folder)?;
    // One write_all of line and newline together, on a file opened in append mode.
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(folder.join("savings.jsonl"))?
        .write_all(format!("{line}\n").as_bytes())
}
