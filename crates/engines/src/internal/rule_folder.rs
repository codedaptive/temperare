//! `rule_folder`: the rule file's folder as the engines print it.

use rules::RuleFile;
use std::fs;
use std::path::{Path, PathBuf};

/// The canonical form of the rule file's folder: `file.folder` when absolute, else joined to `repo`. Canonical
/// because the engines print canonical paths, and a hit's path is made relative to this folder.
pub(crate) fn rule_folder(file: &RuleFile, repo: &Path) -> std::io::Result<PathBuf> {
    // `Path::join` with an absolute path gives that path, which is the absolute case.
    fs::canonicalize(repo.join(&file.folder))
}
