//! `has_path_separator`: whether a program names a path rather than a name to look up on PATH.

use std::ffi::OsStr;

/// True when `program` holds a path separator anywhere, so it is a path and PATH is not consulted.
pub(crate) fn has_path_separator(program: &OsStr) -> bool {
    program.to_string_lossy().chars().any(std::path::is_separator)
}
