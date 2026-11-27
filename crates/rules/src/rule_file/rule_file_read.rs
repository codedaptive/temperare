//! `RuleFile::read`: read and check a rule file from disk.

use super::RuleFile;
use crate::Problem;
use std::path::Path;

impl RuleFile {
    /// Reads the file at `path` and parses it with its folder (an empty parent is "."). An unreadable file is one
    /// Problem at line 0, col 0.
    pub fn read(path: &Path) -> Result<RuleFile, Vec<Problem>> {
        let text = std::fs::read_to_string(path).map_err(|e| {
            vec![Problem {
                line: 0,
                col: 0,
                what: format!("cannot read {}: {e}", path.display()),
                fix: "Pass the path of a readable tao.toml.".to_string(),
            }]
        })?;
        let folder = match path.parent() {
            Some(parent) if !parent.as_os_str().is_empty() => parent,
            _ => Path::new("."),
        };
        RuleFile::parse(&text, folder)
    }
}
