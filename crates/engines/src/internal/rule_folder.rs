//! `rule_folder`: the rule file's canonical folder, the folder a hit's path is placed under.

use rules::RuleFile;
use std::fs;
use std::path::{Path, PathBuf};

/// The canonical form of the rule file's folder: `file.folder` when absolute, else joined to `repo`. Canonical so
/// links are resolved as the engines resolve them, since a hit's path is made relative to this folder; on Windows
/// canonical is the verbatim form (`\\?\C:\...`), which `rel_to_folder` compares in plain disk form.
pub(crate) fn rule_folder(file: &RuleFile, repo: &Path) -> std::io::Result<PathBuf> {
    // `Path::join` with an absolute path gives that path, which is the absolute case.
    fs::canonicalize(repo.join(&file.folder))
}
