//! ZUT for folder_from_top.rs.

use crate::folder_from_top;
use std::path::{Path, PathBuf};

#[test]
fn the_top_itself_is_the_empty_path() {
    assert_eq!(folder_from_top(Path::new("/repo"), Path::new("/repo")), Some(PathBuf::new()));
}

#[test]
fn a_folder_under_the_top_is_the_rest_of_its_path() {
    assert_eq!(folder_from_top(Path::new("/repo"), Path::new("/repo/crates/x")), Some(PathBuf::from("crates/x")));
}

#[test]
fn a_folder_outside_the_top_is_none() {
    assert_eq!(folder_from_top(Path::new("/repo"), Path::new("/other/x")), None);
    assert_eq!(folder_from_top(Path::new("/repo/sub"), Path::new("/repo")), None);
}

#[test]
fn a_sibling_sharing_the_name_text_is_none() {
    assert_eq!(folder_from_top(Path::new("/repo"), Path::new("/repox/a")), None);
}
