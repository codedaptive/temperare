//! `RelPath::new`: a normalised relative path.

use super::RelPath;

impl RelPath {
    /// `path` with every '\\' turned into '/' and every leading "./" removed; nothing else changes, so the same file
    /// written either way compares equal.
    pub fn new(path: &str) -> RelPath {
        let slashed = path.replace('\\', "/");
        let mut rest = slashed.as_str();
        while let Some(stripped) = rest.strip_prefix("./") {
            rest = stripped;
        }
        RelPath(rest.to_string())
    }
}
