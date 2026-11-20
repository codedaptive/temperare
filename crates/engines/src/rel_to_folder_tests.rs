//! Unit tests for rel_to_folder.rs: absolute and relative engine paths made relative to the rule file's folder.

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
