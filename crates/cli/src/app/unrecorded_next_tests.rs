//! ZUT for unrecorded_next.rs: safe removal instructions for added files and folders.

use std::path::PathBuf;

use super::unrecorded_next::unrecorded_next;
use crate::RecordEntry;

fn file(path: &str) -> RecordEntry {
    RecordEntry::File(PathBuf::from(path))
}

#[test]
fn no_earlier_record_and_nothing_placed_says_nothing_was_placed() {
    assert_eq!(
        unrecorded_next(&[], &[]),
        "The install record was not written, and this run placed nothing. Fix the cause and run cctao install \
         again."
    );
}

#[test]
fn no_earlier_record_separates_files_from_folders() {
    let placed = [RecordEntry::Dir(PathBuf::from("/h/.local/state/cctao")), file("/h/.local/bin/cctao")];
    assert_eq!(
        unrecorded_next(&[], &placed),
        "The install record was not written, so cctao uninstall will not remove what this run placed. Remove these \
         by hand: /h/.local/bin/cctao; then remove each of these folders only if it is empty: \
         /h/.local/state/cctao; or fix the cause and run cctao install again."
    );
}

#[test]
fn added_folders_are_listed_in_reverse_order_and_only_if_empty() {
    let placed = [RecordEntry::Dir(PathBuf::from("/h/a")), RecordEntry::Dir(PathBuf::from("/h/a/b"))];
    assert_eq!(
        unrecorded_next(&[], &placed),
        "The install record was not written, so cctao uninstall will not remove what this run placed. Remove each \
         of these folders by hand only if it is empty: /h/a/b, /h/a; or fix the cause and run cctao install again."
    );
}

#[test]
fn earlier_record_with_only_new_folders_lists_each_once_in_reverse_order() {
    let earlier = [RecordEntry::Dir(PathBuf::from("/h"))];
    let placed = [
        RecordEntry::Dir(PathBuf::from("/h")),
        RecordEntry::Dir(PathBuf::from("/h/a")),
        RecordEntry::Dir(PathBuf::from("/h/a/b")),
        RecordEntry::Dir(PathBuf::from("/h/a")),
    ];
    assert_eq!(
        unrecorded_next(&earlier, &placed),
        "The install record was not written; the earlier record is kept, so cctao uninstall covers what it names \
         but not what this run added. Remove each of these folders by hand only if it is empty: /h/a/b, /h/a; or fix \
         the cause and run cctao install again."
    );
}

#[test]
fn an_earlier_record_with_nothing_added_still_covers_the_run() {
    let earlier = [file("/h/.local/bin/cctao"), file("/h/.claude/skills/cctao/SKILL.md")];
    let placed =
        [file("/h/.local/bin/cctao"), file("/h/.claude/skills/cctao/SKILL.md"), file("/h/.local/bin/cctao")];
    assert_eq!(
        unrecorded_next(&earlier, &placed),
        "The install record was not written; the earlier record is kept and names everything this run placed, so \
         cctao uninstall still covers it. Fix the cause and run cctao install again."
    );
}

#[test]
fn an_earlier_record_lists_only_what_this_run_added() {
    let earlier = [file("/h/.local/bin/cctao")];
    let placed = [file("/h/.local/bin/cctao"), file("/h/.codex/skills/cctao/SKILL.md")];
    assert_eq!(
        unrecorded_next(&earlier, &placed),
        "The install record was not written; the earlier record is kept, so cctao uninstall covers what it names \
         but not what this run added. Remove these by hand: /h/.codex/skills/cctao/SKILL.md; or fix the cause and run \
         cctao install again."
    );
}

#[test]
fn a_duplicate_added_entry_is_listed_once_and_an_earlier_one_pushed_again_not_at_all() {
    let earlier = [file("/h/.local/bin/cctao")];
    let placed = [
        file("/h/.local/bin/cctao"),
        file("/h/b"),
        file("/h/.local/bin/cctao"),
        file("/h/b"),
    ];
    assert_eq!(
        unrecorded_next(&earlier, &placed),
        "The install record was not written; the earlier record is kept, so cctao uninstall covers what it names \
         but not what this run added. Remove these by hand: /h/b; or fix the cause and run cctao install again."
    );
}

#[test]
fn placed_order_is_kept() {
    let placed = [
        file("/h/z"),
        file("/h/m"),
        file("/h/a"),
    ];
    assert_eq!(
        unrecorded_next(&[], &placed),
        "The install record was not written, so cctao uninstall will not remove what this run placed. Remove these \
         by hand: /h/z, /h/m, /h/a; or fix the cause and run cctao install again."
    );
}
