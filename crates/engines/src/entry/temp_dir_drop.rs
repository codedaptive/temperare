//! `Drop for TempDir`: removes the temporary folder and the file this value created in it.

use super::TempDir;

impl Drop for TempDir {
    /// Removes the file when this value created it (never a file that was already there), then the folder, which is
    /// removed only when empty, so nothing else put there is lost. What is already gone, or cannot be removed, is
    /// left as it is: a drop cannot report.
    fn drop(&mut self) {
        if let Some(file) = &self.file {
            let _ = std::fs::remove_file(file);
        }
        let _ = std::fs::remove_dir(&self.path);
    }
}
