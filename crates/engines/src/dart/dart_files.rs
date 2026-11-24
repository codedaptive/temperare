//! `dart_files`: the Dart sources under a rule's folder. No unit-test sibling: it is `source_files`, which runs git
//! and reads the real filesystem, so tests/dart.rs covers it.

use super::is_dart_source;
use crate::source_files::source_files;
use crate::types::Stop;
use std::path::{Path, PathBuf};

/// `source_files` keeping the files `is_dart_source` says yes to: tracked, and untracked but not ignored, relative to
/// `folder`, never a symlink.
pub(crate) fn dart_files(folder: &Path) -> Result<Vec<PathBuf>, Stop> {
    source_files(folder, is_dart_source)
}
