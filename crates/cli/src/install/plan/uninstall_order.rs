//! `uninstall_order`: the order uninstall removes the recorded entries in, the install record itself last.

use std::cmp::Reverse;
use std::path::{Path, PathBuf};

use crate::RecordEntry;

/// Two lists, removed one after the other. The first is every file and link in the order given (the record's own
/// entry left out), then the folders that do not hold the record deepest first, then `File(record)` last. The
/// second is the folders that hold the record (each `Dir(dir)` with `record.starts_with(dir)`), deepest first.
/// Deepest first is most path components, folders of equal depth in the order given, so a folder is emptied of
/// what install put in it before its own turn comes. A failure in the first list comes before the record is
/// removed or in removing it, so the record is still there to uninstall again from.
pub(crate) fn uninstall_order(entries: &[RecordEntry], record: &Path) -> (Vec<RecordEntry>, Vec<RecordEntry>) {
    let mut first: Vec<RecordEntry> = Vec::new();
    let mut other_dirs: Vec<PathBuf> = Vec::new();
    let mut record_dirs: Vec<PathBuf> = Vec::new();
    for entry in entries {
        match entry {
            RecordEntry::File(path) if path == record => {}
            RecordEntry::Dir(dir) if record.starts_with(dir) => record_dirs.push(dir.clone()),
            RecordEntry::Dir(dir) => other_dirs.push(dir.clone()),
            RecordEntry::File(_) | RecordEntry::Link(..) => first.push(entry.clone()),
        }
    }
    first.extend(deepest_first(other_dirs));
    first.push(RecordEntry::File(record.to_path_buf()));
    (first, deepest_first(record_dirs).collect())
}

/// `dirs` as `Dir` entries, most path components first; a stable sort keeps equal depths in the order given.
fn deepest_first(mut dirs: Vec<PathBuf>) -> impl Iterator<Item = RecordEntry> {
    dirs.sort_by_key(|dir| Reverse(dir.components().count()));
    dirs.into_iter().map(RecordEntry::Dir)
}
