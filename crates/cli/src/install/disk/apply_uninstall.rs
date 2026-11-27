//! `apply_uninstall`: removes the recorded entries from the disk.
//!
//! No sibling ZUT: it removes from the disk, which a unit test may not (rule 11), so the zone integration target
//! `tests/install` (uninstall.rs) proves it on a temporary HOME.

use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::RecordEntry;

/// Removes what the record names, in the order given (`uninstall_order` sets it): a file entry by its path when it
/// is a file or a symlink (install's own names: the binary, the skill files, the record); a folder only when it is
/// empty. A missing entry is skipped. One line on `out` per removal. Write errors on `out` are ignored.
pub(crate) fn apply_uninstall(entries: &[RecordEntry], out: &mut dyn Write) -> io::Result<()> {
    for entry in entries {
        let removed = match entry {
            RecordEntry::File(path) => remove_file(path)?,
            RecordEntry::Dir(path) => remove_empty_dir(path)?,
        };
        if let Some(path) = removed {
            let _ = writeln!(out, "removed {}", path.display());
        }
    }
    Ok(())
}

/// Removes `path` when it is a file or a symlink; `Some(path)` when it did.
fn remove_file(path: &Path) -> io::Result<Option<&Path>> {
    let Some(meta) = present(path)? else {
        return Ok(None);
    };
    if !(meta.is_file() || meta.file_type().is_symlink()) {
        return Ok(None);
    }
    fs::remove_file(path)?;
    Ok(Some(path))
}

/// Removes `path` when it is a folder with nothing in it; `Some(path)` when it did.
fn remove_empty_dir(path: &Path) -> io::Result<Option<&Path>> {
    let Some(meta) = present(path)? else {
        return Ok(None);
    };
    if !meta.is_dir() || fs::read_dir(path)?.next().is_some() {
        return Ok(None);
    }
    fs::remove_dir(path)?;
    Ok(Some(path))
}

/// `path`'s own metadata (a link is not followed), or `None` when nothing is there.
fn present(path: &Path) -> io::Result<Option<fs::Metadata>> {
    match fs::symlink_metadata(path) {
        Ok(meta) => Ok(Some(meta)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}
