//! Unit test for uninstall_order.rs: the first list is files in order, then the folders that do not hold the
//! record, deepest first, then the record last; the second is the folders that hold the record, deepest first.

use std::path::{Path, PathBuf};

use super::uninstall_order;
use crate::RecordEntry;

const RECORD: &str = "/h/.local/state/cctao/installed";

fn file(path: &str) -> RecordEntry {
    RecordEntry::File(PathBuf::from(path))
}

fn dir(path: &str) -> RecordEntry {
    RecordEntry::Dir(PathBuf::from(path))
}

#[test]
fn a_whole_install_removes_the_record_last_and_its_folders_after_it() {
    let entries = [
        dir("/h/.local"),
        dir("/h/.local/state"),
        dir("/h/.local/state/cctao"),
        dir("/h/.local/bin"),
        file("/h/.local/bin/cctao"),
        dir("/h/.claude/skills/cctao"),
        file("/h/.claude/skills/cctao/SKILL.md"),
    ];
    let (first, second) = uninstall_order(&entries, Path::new(RECORD));
    assert_eq!(
        first,
        vec![
            file("/h/.local/bin/cctao"),
            file("/h/.claude/skills/cctao/SKILL.md"),
            dir("/h/.claude/skills/cctao"),
            dir("/h/.local/bin"),
            file("/h/.local/state/cctao/installed"),
        ]
    );
    assert_eq!(second, vec![dir("/h/.local/state/cctao"), dir("/h/.local/state"), dir("/h/.local")]);
}

#[test]
fn a_recorded_record_is_given_once_at_the_end_of_the_first_list() {
    let entries = [file("/h/.local/state/cctao/installed"), file("/h/a")];
    let (first, second) = uninstall_order(&entries, Path::new(RECORD));
    assert_eq!(first, vec![file("/h/a"), file("/h/.local/state/cctao/installed")]);
    assert_eq!(second, vec![]);
}

#[test]
fn a_folder_that_only_shares_a_name_prefix_with_the_record_is_not_its_ancestor() {
    let entries = [dir("/h/.local/sta"), dir("/h/.local/state/temp"), dir("/h/.local/state")];
    let (first, second) = uninstall_order(&entries, Path::new(RECORD));
    assert_eq!(
        first,
        vec![dir("/h/.local/state/temp"), dir("/h/.local/sta"), file("/h/.local/state/cctao/installed")]
    );
    assert_eq!(second, vec![dir("/h/.local/state")]);
}

#[test]
fn folders_of_equal_depth_keep_their_order() {
    let entries = [dir("/h/b"), dir("/h/a"), dir("/h/a/x"), dir("/h/c")];
    let (first, _) = uninstall_order(&entries, Path::new(RECORD));
    assert_eq!(
        first,
        vec![dir("/h/a/x"), dir("/h/b"), dir("/h/a"), dir("/h/c"), file("/h/.local/state/cctao/installed")]
    );
}

#[test]
fn the_records_folders_go_deepest_first_whatever_order_they_were_given_in() {
    let entries = [dir("/h/.local/state"), dir("/h"), dir("/h/.local")];
    let (_, second) = uninstall_order(&entries, Path::new(RECORD));
    assert_eq!(second, vec![dir("/h/.local/state"), dir("/h/.local"), dir("/h")]);
}

#[test]
fn nothing_recorded_still_removes_the_record() {
    assert_eq!(uninstall_order(&[], Path::new(RECORD)), (vec![file("/h/.local/state/cctao/installed")], vec![]));
}
