//! `kotlin_files`: the Kotlin sources under a rule's folder that detekt is given. No unit-test sibling: it is
//! `source_files`, which runs git and reads the real filesystem, so tests/detekt.rs covers it.

use crate::source_files::source_files;
use crate::types::Stop;
use std::path::{Path, PathBuf};

/// `source_files` keeping the files whose extension is exactly `kt` or `kts`: tracked, and untracked but not ignored,
/// relative to `folder`, never a symlink. What git ignores (build output, generated code) is never listed, so detekt
/// never reads it.
pub(crate) fn kotlin_files(folder: &Path) -> Result<Vec<PathBuf>, Stop> {
    source_files(folder, is_kotlin_source)
}

/// True when `path`'s extension is exactly `kt` or `kts`.
fn is_kotlin_source(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "kt" || ext == "kts")
}
