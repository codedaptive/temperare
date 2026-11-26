//! `pathext`: this process's PATHEXT as `which` reads it, or no PATHEXT rules off Windows.
//!
//! It reads the environment, so it is owed no ZUT: `which_tests.rs` and `tests/which_in_dir.rs` prove it on unix (both are unix-only).

use std::ffi::OsString;

/// This process's PATHEXT, read on Windows only; unset reads as empty, which `names_to_try` maps to its default.
#[cfg(windows)]
pub(crate) fn pathext() -> Option<OsString> {
    Some(std::env::var_os("PATHEXT").unwrap_or_default())
}

/// No PATHEXT rules off Windows: the variable is not read, and a name is tried as given.
#[cfg(not(windows))]
pub(crate) fn pathext() -> Option<OsString> {
    None
}
