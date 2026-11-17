//! `path_from_bytes`: a path made from the exact bytes git printed.

use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;

/// A path made from the exact bytes git printed, non-UTF-8 bytes included.
///
/// Unix only: Temperare runs on macOS and Linux, where a path is any byte string without NUL.
pub(crate) fn path_from_bytes(bytes: Vec<u8>) -> PathBuf {
    PathBuf::from(OsString::from_vec(bytes))
}
