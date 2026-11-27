//! ZUT for rel_to_folder.rs: absolute and relative engine paths made relative to the rule file's folder, with `.`
//! dropped, `..` folded before the folder check, and verbatim folders handled by platform.

use crate::rel_to_folder::rel_to_folder;
use rules::RelPath;
use std::path::Path;

#[test]
fn an_absolute_path_inside_the_folder() {
    let got = rel_to_folder(Path::new("/w/repo"), Path::new("/w/repo"), "/w/repo/src/a.rs");
    assert_eq!(got, Some(RelPath::new("src/a.rs")));
}

#[test]
fn a_relative_path_with_dot_components() {
    let got = rel_to_folder(Path::new("/w/repo"), Path::new("/w/repo"), "./src/./a.py");
    assert_eq!(got, Some(RelPath::new("src/a.py")));
}

#[test]
fn a_cwd_above_the_folder_drops_the_folder_part() {
    let got = rel_to_folder(Path::new("/w/crate"), Path::new("/w/crate/sub"), "sub/src/lib.rs");
    assert_eq!(got, Some(RelPath::new("src/lib.rs")));
}

#[test]
fn a_relative_path_outside_the_folder_is_none() {
    assert_eq!(rel_to_folder(Path::new("/w/crate"), Path::new("/w/crate/sub"), "other/lib.rs"), None);
}

#[test]
fn an_absolute_path_outside_the_folder_is_none() {
    assert_eq!(rel_to_folder(Path::new("/w/repo"), Path::new("/w/repo"), "/elsewhere/a.rs"), None);
}

#[test]
fn a_sibling_folder_sharing_the_name_prefix_is_outside() {
    assert_eq!(rel_to_folder(Path::new("/w"), Path::new("/w/repo"), "/w/repo2/a.rs"), None);
}

#[test]
fn dot_dot_is_folded_before_the_folder_check() {
    let rows: [(&str, &str, &str, &str, Option<&str>); 5] = [
        ("relative_dot_dot_leaves_the_folder", "/w/crate", "/w/crate/sub", "sub/../../x/lib.rs", None),
        ("leading_dot_dot_from_the_folder", "/w/repo", "/w/repo", "../x.rs", None),
        ("absolute_dot_dot_leaves_the_folder", "/w/repo", "/w/repo", "/w/repo/sub/../../other/x.h", None),
        ("absolute_dot_dot_stays_inside", "/w/repo", "/w/repo", "/w/repo/src/../include/x.h", Some("include/x.h")),
        ("cwd_below_the_folder_dot_dot_back_in", "/w/repo/sub", "/w/repo", "../a.rs", Some("a.rs")),
    ];
    for (name, cwd, folder, raw, want) in rows {
        let got = rel_to_folder(Path::new(cwd), Path::new(folder), raw);
        assert_eq!(got, want.map(RelPath::new), "row {name}");
    }
}

#[cfg(windows)]
#[test]
fn a_plain_disk_path_is_placed_under_a_verbatim_folder() {
    let rows: [(&str, &str, &str, &str, Option<&str>); 6] = [
        ("absolute plain under verbatim", r"\\?\C:\x", r"\\?\C:\x", r"C:\x\a.py", Some("a.py")),
        ("relative under verbatim cwd and folder", r"\\?\C:\x", r"\\?\C:\x", r"sub\a.py", Some("sub/a.py")),
        ("plain cwd, verbatim folder", r"C:\x", r"\\?\C:\x", r"a.py", Some("a.py")),
        ("lowercase drive letter", r"\\?\C:\x", r"\\?\C:\x", r"c:\x\a.py", Some("a.py")),
        ("outside", r"\\?\C:\x", r"\\?\C:\x", r"C:\y\a.py", None),
        ("plain dot dot climbing out", r"\\?\C:\x", r"\\?\C:\x", r"C:\x\..\y\a.py", None),
    ];
    for (name, cwd, folder, raw, want) in rows {
        let got = rel_to_folder(Path::new(cwd), Path::new(folder), raw);
        assert_eq!(got, want.map(RelPath::new), "row {name}");
    }
}

#[cfg(unix)]
#[test]
fn a_verbatim_folder_is_not_stripped_on_unix() {
    let rows: [(&str, &str, &str, &str, Option<&str>); 1] = [
        ("unix keeps verbatim text", r"", r"\\?\C:\x", r"C:\x/a.py", None),
    ];
    for (name, cwd, folder, raw, want) in rows {
        let got = rel_to_folder(Path::new(cwd), Path::new(folder), raw);
        assert_eq!(got, want.map(RelPath::new), "row {name}");
    }
}
