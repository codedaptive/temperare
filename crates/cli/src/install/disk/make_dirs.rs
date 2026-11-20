//! `make_dirs`: a folder and its missing ancestors, each one created put on the install record.
//!
//! No sibling unit test: it writes to the disk, which a unit test may not, so the zone integration target
//! `tests/install` (uninstall.rs: a folder install made goes, one that was there stays) proves it on a temporary HOME.

use std::fs;
use std::io;
use std::path::Path;

use crate::RecordEntry;

/// Creates `dir` and each missing ancestor, outermost first, and pushes `RecordEntry::Dir` for each folder it
/// created and only those: uninstall removes what install made, never a folder that was already there.
pub(crate) fn make_dirs(dir: &Path, placed: &mut Vec<RecordEntry>) -> io::Result<()> {
    let missing: Vec<&Path> = dir.ancestors().take_while(|folder| !folder.exists()).collect();
    for folder in missing.into_iter().rev() {
        fs::create_dir(folder)?;
        placed.push(RecordEntry::Dir(folder.to_path_buf()));
    }
    Ok(())
}
