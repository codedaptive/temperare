//! `linted_files`: the files ESLint's JSON output names, relative to the folder it ran in.

use crate::rel_to_folder::rel_to_folder;
use procio::Output;
use rules::RelPath;
use serde_json::Value;
use std::path::Path;

/// The files of `out`'s stdout array, relative to `folder` (where ESLint ran), those outside it left out; None when
/// stdout is not a JSON array.
pub(crate) fn linted_files(out: &Output, folder: &Path) -> Option<Vec<RelPath>> {
    let files: Vec<Value> = serde_json::from_slice(&out.stdout).ok()?;
    Some(files.iter().filter_map(|file| rel_to_folder(folder, folder, file.get("filePath")?.as_str()?)).collect())
}
