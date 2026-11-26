//! `path_from_bytes`: a path made from the bytes git printed.

#[cfg(unix)]
use std::ffi::OsString;
#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;

/// A path made from the exact bytes git printed, non-UTF-8 bytes included.
///
/// On unix a path is any byte string without NUL, so the bytes are kept as they are.
#[cfg(unix)]
pub(crate) fn path_from_bytes(bytes: Vec<u8>) -> PathBuf {
    PathBuf::from(OsString::from_vec(bytes))
}

/// A path made from the bytes git printed, read as UTF-8 (git for Windows prints UTF-8).
///
/// Invalid bytes become U+FFFD, so two different non-UTF-8 names can give the same path, and such a path is never
/// the exact bytes git printed.
#[cfg(not(unix))]
pub(crate) fn path_from_bytes(bytes: Vec<u8>) -> PathBuf {
    PathBuf::from(String::from_utf8_lossy(&bytes).into_owned())
}
