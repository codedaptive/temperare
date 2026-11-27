//! `parse_record`: the install record's text back to its entries.

use std::path::PathBuf;

use crate::RecordEntry;

/// The entries in `text`, the form `record_text` writes, in order and each once. A line that is not `file <path>` or
/// `dir <path>` with an absolute path is skipped, a `link` line among them: uninstall removes what the record names,
/// so it never acts on a path it cannot place.
pub(crate) fn parse_record(text: &str) -> Vec<RecordEntry> {
    let mut entries = Vec::new();
    for entry in text.lines().filter_map(entry) {
        if !entries.contains(&entry) {
            entries.push(entry);
        }
    }
    entries
}

/// The entry one line names, or `None` for any other line.
fn entry(line: &str) -> Option<RecordEntry> {
    let (word, rest) = line.split_once(' ')?;
    match word {
        "file" => absolute(rest).map(RecordEntry::File),
        "dir" => absolute(rest).map(RecordEntry::Dir),
        _ => None,
    }
}

/// `text` as a path when it is absolute.
fn absolute(text: &str) -> Option<PathBuf> {
    Some(PathBuf::from(text)).filter(|path| path.is_absolute())
}
