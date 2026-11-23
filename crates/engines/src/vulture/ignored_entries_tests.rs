use super::ignored_entries::ignored_entries;
use std::path::{Path, PathBuf};

#[test]
fn ignored_entries_filters_and_preserves_listing_order() {
    let cases: &[(&str, &[u8], &str, &[(&str, bool)])] = &[
        ("folder", b".venv/\0", "/r", &[("/r/.venv", true)]),
        ("python", b"gen/out.py\0", "/r", &[("/r/gen/out.py", false)]),
        ("cache", b"pkg/__pycache__/\0", "/r", &[("/r/pkg/__pycache__", true)]),
        ("other files", b"a.pyc\0notes.txt\0", "/r", &[]),
        ("outside", b"other/x.py\0", "/r/sub", &[]),
        ("same folder", b"sub/\0", "/r/sub", &[]),
        ("empty", b"", "/r", &[]),
        ("order and empty parts", b"\0gen/out.py\0.venv/\0\0", "/r", &[("/r/gen/out.py", false), ("/r/.venv", true)]),
    ];
    let mut failed = Vec::new();
    for &(name, listing, folder, expected) in cases {
        let result = std::panic::catch_unwind(|| {
            let expected: Vec<_> = expected.iter().map(|(path, is_folder)| (PathBuf::from(path), *is_folder)).collect();
            assert_eq!(ignored_entries(listing, Path::new("/r"), Path::new(folder)), expected, "{name}");
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
