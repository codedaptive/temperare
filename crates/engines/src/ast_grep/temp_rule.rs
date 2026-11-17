//! `TempRule`: a filled rule YAML in a temporary file that lives as long as the value.

use std::path::PathBuf;

/// A rule file written for one `ast-grep scan --rule` run; dropping it removes the file.
#[derive(Debug)]
pub struct TempRule {
    /// The file, under the system's temporary folder.
    pub path: PathBuf,
}
