use super::ignored_entries::ignored_entries;
use std::path::{Path, PathBuf};

#[test]
fn ignored_entries_filters_and_preserves_listing_order() {
    let cases: &[(&str, &[&str], &str, &[(&str, bool)])] = &[
        ("folder", &[".venv/"], "/r", &[("/r/.venv", true)]),
        ("python", &["gen/out.py"], "/r", &[("/r/gen/out.py", false)]),
        ("cache", &["pkg/__pycache__/"], "/r", &[("/r/pkg/__pycache__", true)]),
        ("other files", &["a.pyc", "notes.txt"], "/r", &[]),
        ("outside", &["other/x.py"], "/r/sub", &[]),
        ("same folder", &["sub/"], "/r/sub", &[]),
        ("empty", &[], "/r", &[]),
        ("order and empty parts", &["", "gen/out.py", ".venv/", ""], "/r", &[("/r/gen/out.py", false), ("/r/.venv", true)]),
        ("lossy name", &["caf\u{FFFD}/", "caf\u{FFFD}.py"], "/r", &[]),
    ];
    let mut failed = Vec::new();
    for &(name, entries, folder, expected) in cases {
        let result = std::panic::catch_unwind(|| {
            let entries: Vec<String> = entries.iter().map(|entry| entry.to_string()).collect();
            let expected: Vec<_> = expected.iter().map(|(path, is_folder)| (PathBuf::from(path), *is_folder)).collect();
            assert_eq!(ignored_entries(&entries, Path::new("/r"), Path::new(folder)), expected, "{name}");
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
