//! `Drop for ScratchDir`: removes the scratch folder and everything in it.

use super::ScratchDir;

impl Drop for ScratchDir {
    /// Removes the folder with all it holds. It is only ever a folder this value created (`ScratchDir::new`), so
    /// nothing else is lost. What cannot be removed is left as it is: a drop cannot report.
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}
