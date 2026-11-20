//! Unit tests for only_paths.rs: relative, `./`, `../`, trailing-slash and absolute paths under the folder, a path outside
//! it, and the folder itself. Pure: nothing is read from disk, so the folders named here need not exist.

use super::only_paths::only_paths;
use crate::UsageError;
use rules::RelPath;
use std::path::{Path, PathBuf};

fn paths(given: &[&str]) -> Vec<PathBuf> {
    given.iter().map(PathBuf::from).collect()
}

fn texts(only: &[RelPath]) -> Vec<&str> {
    only.iter().map(|p| p.as_str()).collect()
}

#[test]
fn no_paths_is_the_whole_tree() {
    assert_eq!(only_paths(&[], Path::new("/repo"), Path::new("/repo")).unwrap(), Vec::<RelPath>::new());
}

#[test]
fn a_relative_folder_is_relative_to_the_rule_file_folder() {
    let only = only_paths(&paths(&["src"]), Path::new("/repo"), Path::new("/repo")).unwrap();
    assert_eq!(texts(&only), vec!["src"]);
}

#[test]
fn a_trailing_slash_is_removed_from_a_folder() {
    let only = only_paths(&paths(&["src/"]), Path::new("/repo"), Path::new("/repo")).unwrap();
    assert_eq!(texts(&only), vec!["src"]);
}

#[test]
fn a_leading_dot_is_removed_from_a_folder() {
    let only = only_paths(&paths(&["./src"]), Path::new("/repo"), Path::new("/repo")).unwrap();
    assert_eq!(texts(&only), vec!["src"]);
}

#[test]
fn dot_alone_at_the_rule_file_folder_is_the_whole_tree() {
    let only = only_paths(&paths(&["."]), Path::new("/repo"), Path::new("/repo")).unwrap();
    assert_eq!(texts(&only), Vec::<&str>::new());
}

#[test]
fn parent_from_a_subfolder_is_the_whole_tree() {
    let only = only_paths(&paths(&[".."]), Path::new("/repo/app"), Path::new("/repo")).unwrap();
    assert_eq!(texts(&only), Vec::<&str>::new());
}

#[test]
fn relative_paths_are_taken_from_cwd_in_the_order_given() {
    let only = only_paths(&paths(&["b.py", "src/a.py"]), Path::new("/repo/app"), Path::new("/repo")).unwrap();
    assert_eq!(texts(&only), vec!["app/b.py", "app/src/a.py"]);
}

#[test]
fn dot_and_dot_dot_are_resolved_without_the_disk() {
    let only =
        only_paths(&paths(&["./a.py", "../lib/./b.py", "x/../c.py"]), Path::new("/repo/app"), Path::new("/repo")).unwrap();
    assert_eq!(texts(&only), vec!["app/a.py", "lib/b.py", "app/c.py"]);
}

#[test]
fn the_folder_is_normalised_too() {
    let only = only_paths(&paths(&["a.py"]), Path::new("/repo/app"), Path::new("/repo/./app/../app")).unwrap();
    assert_eq!(texts(&only), vec!["a.py"]);
}

#[test]
fn an_absolute_path_inside_the_folder_stays_as_given() {
    let only = only_paths(&paths(&["/repo/src/a.py"]), Path::new("/elsewhere"), Path::new("/repo")).unwrap();
    assert_eq!(texts(&only), vec!["src/a.py"]);
}

#[test]
fn a_path_outside_the_folder_is_a_usage_error() {
    let got = only_paths(&paths(&["a.py", "../other/x.py"]), Path::new("/repo/app"), Path::new("/repo/app"));
    assert_eq!(
        got,
        Err(UsageError {
            what: "../other/x.py is not under the rule file's folder /repo/app".to_string(),
            fix: "Pass paths inside /repo/app, or pass --rules with the rule file that covers them.".to_string(),
        })
    );
}

#[test]
fn a_sibling_folder_sharing_a_name_prefix_is_outside() {
    let got = only_paths(&paths(&["/repo-two/a.py"]), Path::new("/repo"), Path::new("/repo"));
    assert_eq!(got.unwrap_err().what, "/repo-two/a.py is not under the rule file's folder /repo");
}

#[test]
fn the_folder_itself_is_the_whole_tree() {
    let only = only_paths(&paths(&["a.py", ".", "b.py"]), Path::new("/repo"), Path::new("/repo")).unwrap();
    assert_eq!(only, Vec::<RelPath>::new());
}

#[cfg(unix)]
#[test]
fn a_path_that_is_not_utf8_is_a_usage_error() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    let bad = PathBuf::from(OsStr::from_bytes(b"a\xff.py"));
    let got = only_paths(&[bad], Path::new("/repo"), Path::new("/repo"));
    assert_eq!(
        got.unwrap_err().fix,
        "Pass paths inside /repo, or pass --rules with the rule file that covers them."
    );
}

#[test]
fn a_path_outside_beside_the_folder_itself_is_still_a_usage_error() {
    let got = only_paths(&paths(&[".", "/other/a.py"]), Path::new("/repo"), Path::new("/repo"));
    assert_eq!(got.unwrap_err().what, "/other/a.py is not under the rule file's folder /repo");
}
