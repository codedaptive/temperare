//! `is_dart_source`: whether a file is a Dart source `dart analyze` checks.

use std::path::Path;

/// True when `path`'s extension is exactly `dart`. Case counts: the analyzer reads `.dart` files only.
pub(crate) fn is_dart_source(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "dart")
}
