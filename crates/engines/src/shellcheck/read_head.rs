//! `read_head`: the first bytes of a regular file, read without following a symlink or blocking on a FIFO. It
//! touches the real filesystem, so its tests are the zone integration tests in integration/read_head_tests.rs and it
//! has no unit-test sibling.

use std::path::Path;

/// The most bytes read: a first line of up to 256 bytes and the byte after it, which tells a 256-byte line ended by
/// `\n` from a longer one.
#[cfg(unix)]
const HEAD_BYTES: u64 = 257;

/// The first 257 bytes of `path`, or all of it when shorter; `None` unless `path` is a regular file that can be read.
/// A symlink is `None` without being followed: `symlink_metadata` checks the path, and the open uses `O_NOFOLLOW`
/// in case the path changes between the two. The open also uses `O_NONBLOCK`, so a path swapped for a FIFO after the
/// check is not waited on; the opened file's own metadata must then say a regular file again. Any error is `None`.
#[cfg(unix)]
pub(crate) fn read_head(path: &Path) -> Option<Vec<u8>> {
    use std::fs::OpenOptions;
    use std::io::Read;
    use std::os::unix::fs::OpenOptionsExt;
    if !std::fs::symlink_metadata(path).ok()?.file_type().is_file() {
        return None;
    }
    let file = OpenOptions::new().read(true).custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK).open(path).ok()?;
    if !file.metadata().ok()?.file_type().is_file() {
        return None;
    }
    let mut head = Vec::new();
    file.take(HEAD_BYTES).read_to_end(&mut head).ok()?;
    Some(head)
}

/// Off unix: always `None`. The symlink-refusing open is unix-only (`O_NOFOLLOW`, from `libc`), so nothing is read and the caller leaves
/// the file out, never `Failed`.
#[cfg(not(unix))]
pub(crate) fn read_head(_path: &Path) -> Option<Vec<u8>> {
    None
}
