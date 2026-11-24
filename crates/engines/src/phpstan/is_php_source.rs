//! `is_php_source`: whether a file is a PHP file PHPStan is given.

use std::ffi::OsStr;
use std::path::Path;

/// True when `path`'s extension is exactly `php`, PHPStan's default `fileExtensions`.
pub(crate) fn is_php_source(path: &Path) -> bool {
    path.extension() == Some(OsStr::new("php"))
}
