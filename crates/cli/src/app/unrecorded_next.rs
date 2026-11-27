//! `unrecorded_next`: the `next:` line `cctao install` writes when its record cannot be written.

use crate::RecordEntry;

/// The `next:` line when the install record is not written. A failed write leaves the earlier record on disk as it
/// was, so uninstall still covers what `earlier` (the record read before any step) names; the line lists what
/// `placed` (the entries after the steps) adds to it, each once. Files are named in `placed` order to remove by
/// hand; folders are named, each before its parent, to remove only if empty, as uninstall does, since a folder may
/// hold files this run did not place.
pub(super) fn unrecorded_next(earlier: &[RecordEntry], placed: &[RecordEntry]) -> String {
    let mut added: Vec<&RecordEntry> = Vec::new();
    for entry in placed {
        if !earlier.contains(entry) && !added.contains(&entry) {
            added.push(entry);
        }
    }
    match (earlier.is_empty(), added.is_empty()) {
        (true, true) => {
            "The install record was not written, and this run placed nothing. Fix the cause and run cctao install \
             again."
                .to_string()
        }
        (true, false) => format!(
            "The install record was not written, so cctao uninstall will not remove what this run placed. {}; or \
             fix the cause and run cctao install again.",
            removal(&added)
        ),
        (false, true) => "The install record was not written; the earlier record is kept and names everything this \
                          run placed, so cctao uninstall still covers it. Fix the cause and run cctao install \
                          again."
            .to_string(),
        (false, false) => format!(
            "The install record was not written; the earlier record is kept, so cctao uninstall covers what it \
             names but not what this run added. {}; or fix the cause and run cctao install again.",
            removal(&added)
        ),
    }
}

/// The removal instruction for the added entries: files in order, then folders in reverse order. The install makes
/// folders outermost first, so reversing puts each folder before its parent.
fn removal(added: &[&RecordEntry]) -> String {
    let mut files: Vec<String> = Vec::new();
    let mut dirs: Vec<String> = Vec::new();
    for entry in added {
        match entry {
            RecordEntry::File(path) => files.push(path.display().to_string()),
            RecordEntry::Dir(path) => dirs.push(path.display().to_string()),
        }
    }
    dirs.reverse();
    match (files.is_empty(), dirs.is_empty()) {
        (false, false) => format!(
            "Remove these by hand: {}; then remove each of these folders only if it is empty: {}",
            files.join(", "),
            dirs.join(", ")
        ),
        (false, true) => format!("Remove these by hand: {}", files.join(", ")),
        // Folders only: `removal` is called only where something was added, so files and folders are never both empty.
        _ => format!("Remove each of these folders by hand only if it is empty: {}", dirs.join(", ")),
    }
}
