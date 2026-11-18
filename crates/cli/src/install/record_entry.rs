//! `RecordEntry`: one line of the install record, the list of what install created and uninstall removes.

use std::path::PathBuf;

/// A file install created (the binary, a skill file, the record itself), the `tm` symlink with the target install
/// gave it (link, target: uninstall removes it only while it still points there), or a folder install created.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordEntry {
    File(PathBuf),
    Link(PathBuf, PathBuf),
    Dir(PathBuf),
}
