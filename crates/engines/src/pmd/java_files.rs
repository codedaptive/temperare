//! `java_files`: the Java sources under a rule's folder that PMD is given. No unit-test sibling: it is
//! `source_files`, which runs git and reads the real filesystem, so tests/pmd.rs covers it.

use crate::source_files::source_files;
use crate::types::Stop;
use std::path::{Path, PathBuf};

/// `source_files` keeping the files whose extension is exactly `java`: tracked, and untracked but not ignored,
/// relative to `folder`, never a symlink.
pub(crate) fn java_files(folder: &Path) -> Result<Vec<PathBuf>, Stop> {
    source_files(folder, is_java_source)
}

/// True when `path`'s extension is exactly `java`.
fn is_java_source(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "java")
}
