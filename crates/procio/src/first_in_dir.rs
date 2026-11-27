//! `first_in_dir`: the one per-folder lookup `which_in` and `which_in_dir` share.
//!
//! It reads the disk, so it is owed no unit test: `which_tests.rs` and `tests/which_in_dir.rs` prove it, and
//! `tests/which_windows.rs` on Windows.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

use crate::is_executable::is_executable;

/// `dir` joined with the first of `names`, in order, that is an executable regular file there; `None` when none is.
pub(crate) fn first_in_dir(dir: &Path, names: &[OsString]) -> Option<PathBuf> {
    names.iter().map(|name| dir.join(name)).find(|candidate| is_executable(candidate))
}
