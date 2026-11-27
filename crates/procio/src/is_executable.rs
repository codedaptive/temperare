//! `is_executable`: whether a path is a file `which` may return.
//!
//! It reads the disk, so it is owed no unit test: `which_tests.rs` and `tests/which_in_dir.rs` prove it on unix
//! (both are unix-only), `tests/which_windows.rs` on Windows.

use std::path::Path;

/// A regular file with an execute bit set for anyone.
#[cfg(unix)]
pub(crate) fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    path.metadata().is_ok_and(|meta| meta.is_file() && meta.permissions().mode() & 0o111 != 0)
}

/// A regular file: outside unix there is no execute bit to read.
#[cfg(not(unix))]
pub(crate) fn is_executable(path: &Path) -> bool {
    path.is_file()
}
