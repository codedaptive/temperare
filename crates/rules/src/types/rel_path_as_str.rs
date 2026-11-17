//! `RelPath::as_str`: the path as text.

use super::RelPath;

impl RelPath {
    /// The normalised path, '/'-separated.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
