#![cfg(unix)]

use std::ffi::OsStr;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Component, Path, PathBuf};

fn scratch(test: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("procio-which-in-dir-{}-{test}", std::process::id()));
    fs::create_dir(&dir).unwrap();
    fs::create_dir(dir.join("non-executable")).unwrap();
    fs::create_dir(dir.join("directory")).unwrap();
    fs::create_dir(dir.join("directory/tool")).unwrap();
    fs::create_dir(dir.join("sub")).unwrap();
    for (path, mode) in [("tool", 0o755), ("non-executable/tool", 0o644), ("sub/tool", 0o755)] {
        fs::write(dir.join(path), "").unwrap();
        fs::set_permissions(dir.join(path), fs::Permissions::from_mode(mode)).unwrap();
    }
    dir
}

/// `path` relative to the current directory: one `..` per cwd component past the common prefix, then the rest of
/// `path`.
fn relative_to_cwd(path: &Path) -> PathBuf {
    let cwd = std::env::current_dir().unwrap();
    let cwd_parts: Vec<Component> = cwd.components().collect();
    let path_parts: Vec<Component> = path.components().collect();
    let common = path_parts.iter().zip(&cwd_parts).take_while(|(a, b)| a == b).count();
    let mut rel = PathBuf::new();
    cwd_parts[common..].iter().for_each(|_| rel.push(".."));
    path_parts[common..].iter().for_each(|part| rel.push(part));
    rel
}

macro_rules! which_in_dir_cases {
    ($dir:ident; $($test:ident: ($lookup:expr, $name:expr, $expected:expr)),+ $(,)?) => {
        $(
            #[test]
            fn $test() {
                let $dir = scratch(stringify!($test));
                let result = procio::which_in_dir(&$lookup, OsStr::new($name));
                let expected = $expected;
                fs::remove_dir_all(&$dir).unwrap();
                assert_eq!(result, expected);
            }
        )+
    };
}

which_in_dir_cases! {
    dir;
    found: (dir, "tool", Some(dir.join("tool"))),
    not_found: (dir, "absent", None),
    non_executable_file: (dir.join("non-executable"), "tool", None),
    directory_named_tool: (dir.join("directory"), "tool", None),
    missing_dir: (dir.join("missing"), "tool", None),
    // On unix stat also rejects `tool/`, so this row checks the result only; the empty-name guard shows only where
    // PATHEXT turns "" into names like `.COM`.
    executable_dir_with_empty_name: (dir.join("tool"), "", None),
    name_with_separator: (dir, "sub/tool", None),
    relative_dir: (relative_to_cwd(&dir), "tool", None),
}
