//! `python_files`: the Python files under a rule's folder that mypy is given. No unit-test sibling: it is
//! `source_files`, which runs git and reads the real filesystem, so tests/mypy.rs covers it.

use crate::source_files::source_files;
use crate::types::Stop;
use std::path::{Path, PathBuf};

/// `source_files` keeping the files whose extension is exactly `py` or `pyi`: tracked, and untracked but not ignored
/// (so a git-ignored `.venv` never reaches mypy), relative to `folder`, never a symlink.
pub(crate) fn python_files(folder: &Path) -> Result<Vec<PathBuf>, Stop> {
    source_files(folder, is_python_source)
}

/// True when `path`'s extension is exactly `py` or `pyi`.
fn is_python_source(path: &Path) -> bool {
    path.extension().is_some_and(|ext| ext == "py" || ext == "pyi")
}
