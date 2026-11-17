use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use super::path_from_bytes;

#[test]
fn utf8_bytes_give_the_same_path() {
    assert_eq!(path_from_bytes(b"src/caf\xC3\xA9.rs".to_vec()), Path::new("src/café.rs"));
}

#[test]
fn non_utf8_bytes_are_kept_exactly() {
    let path = path_from_bytes(vec![b'a', 0xFF, b'/', 0x80]);
    assert_eq!(path.as_os_str().as_bytes(), &[b'a', 0xFF, b'/', 0x80]);
}
