//! Unit test for strip_verbatim_disk.rs: the verbatim `\\?\` prefix dropped from disk roots only.

use crate::strip_verbatim_disk::strip_verbatim_disk;
use std::path::Path;

#[test]
fn verbatim_disk_prefix_is_stripped_only_from_disk_roots() {
    for (path, expected) in [
        (r"\\?\C:\work", r"C:\work"),
        (r"\\?\c:\w\x", r"c:\w\x"),
        (r"\\?\UNC\srv\share", r"\\?\UNC\srv\share"),
        (r"\\?\C:", r"\\?\C:"),
        (r"C:\work", r"C:\work"),
        ("/work/project", "/work/project"),
    ] {
        assert_eq!(strip_verbatim_disk(Path::new(path)), Path::new(expected), "{path:?}");
    }
}
