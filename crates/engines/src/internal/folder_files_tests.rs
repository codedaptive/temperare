//! Unit test for folder_files.rs: listings shaped as ccvcslib gives them (top-relative) made by hand, cut to the rule file's
//! folder.

use crate::folder_files::folder_files;
use rules::RelPath;
use std::path::{Path, PathBuf};

const TOP: &str = "/repo";
const FOLDER: &str = "/repo/sub";

fn paths(list: &[&str]) -> Vec<PathBuf> {
    list.iter().map(PathBuf::from).collect()
}

fn rel(list: &[&str]) -> Vec<RelPath> {
    list.iter().map(|path| RelPath::new(path)).collect()
}

fn files(tracked: &[&str], untracked: &[&str]) -> Vec<RelPath> {
    folder_files(Path::new(TOP), Path::new(FOLDER), &paths(tracked), &paths(untracked))
}

#[test]
fn an_untracked_file_in_the_folder_is_kept() {
    assert_eq!(files(&[], &["sub/lib/b.py"]), rel(&["lib/b.py"]));
}

#[test]
fn tracked_files_come_first_then_untracked() {
    assert_eq!(files(&["sub/src/a.py"], &["sub/lib/b.py"]), rel(&["src/a.py", "lib/b.py"]));
}

#[test]
fn an_untracked_path_outside_the_folder_is_dropped() {
    assert_eq!(files(&["sub/src/a.py"], &["other/x.py", "subway/y.py", "sub/lib/b.py"]), rel(&["src/a.py", "lib/b.py"]));
}

#[test]
fn a_path_in_both_listings_appears_once() {
    assert_eq!(files(&["sub/src/a.py", "sub/lib/b.py"], &["sub/lib/b.py"]), rel(&["src/a.py", "lib/b.py"]));
}

#[test]
fn a_directory_entry_is_dropped_and_a_file_under_that_folder_is_kept() {
    // git prints an untracked nested repository as one `inner/` entry; a tracked file never ends in '/'.
    let cases: [(&[&str], &[&str], &[&str]); 3] = [
        (&[], &["sub/inner/"], &[]),
        (&["sub/inner/a.py"], &[], &["inner/a.py"]),
        (&["sub/src/a.py"], &["sub/inner/", "sub/lib/b.py"], &["src/a.py", "lib/b.py"]),
    ];
    for (tracked, untracked, want) in cases {
        assert_eq!(files(tracked, untracked), rel(want), "tracked {tracked:?}, untracked {untracked:?}");
    }
}

#[test]
fn nothing_listed_gives_nothing() {
    assert_eq!(files(&[], &[]), Vec::<RelPath>::new());
}
