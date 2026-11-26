#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use super::path_from_bytes;

#[test]
fn utf8_bytes_give_the_same_path() {
    assert_eq!(path_from_bytes(b"src/caf\xC3\xA9.rs".to_vec()), Path::new("src/café.rs"));
}

// Unix only: it reads the path's raw bytes back, which only unix's OsStr gives.
#[cfg(unix)]
#[test]
fn non_utf8_bytes_are_kept_exactly() {
    let path = path_from_bytes(vec![b'a', 0xFF, b'/', 0x80]);
    assert_eq!(path.as_os_str().as_bytes(), &[b'a', 0xFF, b'/', 0x80]);
}

// Off unix only: it checks the lossy UTF-8 branch, which unix never compiles.
#[cfg(not(unix))]
#[test]
fn non_utf8_bytes_become_replacement_characters_off_unix() {
    assert_eq!(path_from_bytes(vec![b'a', 0xFF, b'/', 0x80]), Path::new("a\u{FFFD}/\u{FFFD}"));
}
