//! `has_lock`: whether a folder holds the Cargo.lock cargo-audit reads. Reads the disk, so it has no unit test:
//! tests/cargo_audit.rs (no Cargo.lock) tests it.

use std::path::Path;

/// True when `<folder>/Cargo.lock` is a regular file itself, not a symlink: a link could point outside the repository.
pub(crate) fn has_lock(folder: &Path) -> bool {
    std::fs::symlink_metadata(folder.join("Cargo.lock")).is_ok_and(|meta| meta.file_type().is_file())
}
