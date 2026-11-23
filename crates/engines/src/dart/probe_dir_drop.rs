//! `Drop for ProbeDir`: removes the probe's files, then its folder. It writes the disk, so its test is the zone
//! integration test tests/dart.rs and it has no unit-test sibling.

use super::ProbeDir;

impl Drop for ProbeDir {
    /// Removes each file the value tried to write (each path in `files`), then the folder, which is then empty unless
    /// something else wrote in it; a folder holding anything else is left, so nothing the value did not write is lost.
    /// What cannot be removed is left as it is: a drop cannot report.
    fn drop(&mut self) {
        for file in &self.files {
            let _ = std::fs::remove_file(file);
        }
        let _ = std::fs::remove_dir(&self.path);
    }
}
