//! `write_case_files`: a case's files written into its case folder.

use super::case_path::case_path;
use super::BaselineWorld;
use rules::CaseFile;
use std::path::Path;

/// Writes each of `files` inside `folder`, in order, at the path `case_path` gives for its name, so only inside that
/// folder: a name that is not a relative path inside it is refused with `case_path`'s why and nothing is written for
/// it. The first file that cannot be placed or written (`cannot write <name>: <e>`) stops the rest.
pub(crate) fn write_case_files(folder: &Path, files: &[CaseFile], world: &BaselineWorld) -> Result<(), String> {
    for file in files {
        let path = case_path(folder, &file.path)?;
        (world.write_file)(&path, file.text).map_err(|e| format!("cannot write {}: {e}", file.path))?;
    }
    Ok(())
}
