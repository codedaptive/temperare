use super::exclude_arg::exclude_arg;
use std::ffi::OsString;
use std::path::PathBuf;

#[test]
fn exclude_arg_encodes_paths_or_reports_unrepresentable_input() {
    let cases = [
        ("empty", vec![], Ok(None)),
        ("folder", vec![(PathBuf::from("/r/.venv"), true)], Ok(Some(OsString::from("--exclude=/r/.venv/*")))),
        ("folder and file", vec![(PathBuf::from("/r/.venv"), true), (PathBuf::from("/r/gen/out.py"), false)], Ok(Some(OsString::from("--exclude=/r/.venv/*,/r/gen/out.p[y]")))),
        ("metacharacters", vec![(PathBuf::from("/r/a[*?.py"), false)], Ok(Some(OsString::from("--exclude=/r/a[[][*][?].p[y]")))),
    ];
    let mut failed = Vec::new();
    for (name, entries, expected) in cases {
        if std::panic::catch_unwind(|| assert_eq!(exclude_arg(&entries), expected, "{name}")).is_err() { failed.push(name); }
    }
    for (name, entries, tokens) in [
        ("comma", vec![(PathBuf::from("/r/a,b.py"), false)], vec!["/r/a,b.py"]),
        ("too long", vec![(PathBuf::from(format!("/r/{}.py", "a".repeat(98304))), false)], vec!["too many", "git-ignored", "Python", "folder"]),
    ] {
        let result = std::panic::catch_unwind(|| {
            let error = exclude_arg(&entries).unwrap_err();
            for token in tokens { assert!(error.contains(token), "{name}: {error:?} lacks {token:?}"); }
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
