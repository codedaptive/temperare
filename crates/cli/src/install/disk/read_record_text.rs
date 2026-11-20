//! `read_record_text`: the text of the install record, refusing anything at its path that is not a plain file.
//!
//! No sibling unit test: it reads the disk, which a unit test may not, so the zone integration target
//! `tests/install` (record.rs, uninstall.rs) proves it on a temporary HOME.

use std::fs::{self, File};
use std::io::{self, Read};
use std::path::Path;

/// The text of the install record at `record`, or `None` when nothing is there. The path is looked at with
/// `symlink_metadata`, so a link is never followed: a symbolic link at `record` is refused as one ("is a symbolic
/// link, not the install record temperare install writes"), and anything else that is not a regular file is
/// refused as "is not a file". A regular file is opened and, on unix, read only when the opened handle's device and
/// inode equal the ones looked at first, so a link or another file swapped in between the look and the open is
/// refused rather than read. Off unix the file is read after the look alone. Each refusal is an `io::Error` of kind
/// `Other` naming `record`.
pub(crate) fn read_record_text(record: &Path) -> io::Result<Option<String>> {
    let looked = match fs::symlink_metadata(record) {
        Ok(looked) => looked,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e),
    };
    if looked.file_type().is_symlink() {
        return Err(refusal(record, "is a symbolic link"));
    }
    if !looked.is_file() {
        return Err(refusal(record, "is not a file"));
    }
    let mut file = File::open(record)?;
    if !same_file(&looked, &file.metadata()?) {
        return Err(refusal(record, "changed while it was being read"));
    }
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(Some(text))
}

/// `<record> <what>, not the install record temperare install writes`.
fn refusal(record: &Path, what: &str) -> io::Error {
    io::Error::new(
        io::ErrorKind::Other,
        format!("{} {what}, not the install record temperare install writes", record.display()),
    )
}

/// Whether the opened handle is the file looked at: the same device and inode.
#[cfg(unix)]
fn same_file(looked: &fs::Metadata, opened: &fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;
    looked.dev() == opened.dev() && looked.ino() == opened.ino()
}

/// Off unix there is no device and inode to compare; the look before the open is the check.
#[cfg(not(unix))]
fn same_file(_looked: &fs::Metadata, _opened: &fs::Metadata) -> bool {
    true
}
