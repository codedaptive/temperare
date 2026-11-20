//! `Drop for TempFile`: removes the temporary file.

use super::TempFile;

impl Drop for TempFile {
    /// Removes the file. A file already gone, or one that cannot be removed, is left as it is: a drop cannot report.
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}
