//! `record_text`: the install record's text, one line per thing install created.

use std::path::Path;

use super::unfit_for_record::unfit_for_record;
use crate::RecordEntry;

/// `file <path>` or `dir <path>` per entry, each line ending in a newline, in the order
/// given; an entry given twice is written once, where it first appears. An entry with a path `unfit_for_record`
/// rejects (not UTF-8, or holding `\n`, `\r` or `\t`) is not written: not UTF-8 it would be written lossily, with `\n`
/// it would read back as other entries, and `\r` and `\t` are refused to keep each line one plain path
/// (`run_install` refuses such a path with `unrecordable_path` before it writes anything).
/// `parse_record` reads the text back.
pub(crate) fn record_text(entries: &[RecordEntry]) -> String {
    let mut seen: Vec<&RecordEntry> = Vec::new();
    let mut text = String::new();
    for entry in entries {
        if seen.contains(&entry) {
            continue;
        }
        seen.push(entry);
        let line = match entry {
            RecordEntry::File(path) => fit_text(path).map(|path| format!("file {path}\n")),
            RecordEntry::Dir(path) => fit_text(path).map(|path| format!("dir {path}\n")),
        };
        text.extend(line);
    }
    text
}

/// The path's own text, or `None` when it is unfit for the record.
fn fit_text(path: &Path) -> Option<&str> {
    path.to_str().filter(|_| !unfit_for_record(path))
}
