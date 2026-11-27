//! `RecordEntry`: one line of the install record, the list of what install created and uninstall removes.

use std::path::PathBuf;

/// A file install created (the binary, a skill file, the record itself), or a folder install created.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordEntry {
    File(PathBuf),
    Dir(PathBuf),
}
