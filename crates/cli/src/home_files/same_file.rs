//! `same_file`: whether an opened handle is the file looked at before the open.
//!
//! No sibling ZUT: its inputs are metadata read from the disk, so home_files/integration/same_file_tests.rs proves the
//! unix comparison (GROUP=integration); the non-unix one is untested.

use std::fs;

/// Whether the opened handle is the file looked at: the same device and inode.
#[cfg(unix)]
pub(crate) fn same_file(looked: &fs::Metadata, opened: &fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;
    looked.dev() == opened.dev() && looked.ino() == opened.ino()
}

/// Off unix there is no device and inode to compare; the look before the open is the check.
#[cfg(not(unix))]
pub(crate) fn same_file(_looked: &fs::Metadata, _opened: &fs::Metadata) -> bool {
    true
}
