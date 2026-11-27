//! Unit test for record_text.rs: one line per entry, in order, each written once; an entry whose path is not UTF-8 or
//! holds a newline, return or tab is left out, so parse_record (pure) reads back no injected entry (that test runs
//! on unix only).

use std::path::PathBuf;

use super::record_text;
use crate::RecordEntry;

fn file(path: &str) -> RecordEntry {
    RecordEntry::File(PathBuf::from(path))
}

fn dir(path: &str) -> RecordEntry {
    RecordEntry::Dir(PathBuf::from(path))
}

#[test]
fn files_and_dirs_one_per_line_in_order() {
    let entries = [dir("/h/.local/bin"), file("/h/.local/bin/cctao"), dir("/h/.local/state")];
    assert_eq!(record_text(&entries), "dir /h/.local/bin\nfile /h/.local/bin/cctao\ndir /h/.local/state\n");
}

#[test]
fn a_repeat_is_written_once_where_it_first_appears() {
    let entries = [file("/h/a"), dir("/h/d"), file("/h/a"), dir("/h/d"), file("/h/b")];
    assert_eq!(record_text(&entries), "file /h/a\ndir /h/d\nfile /h/b\n");
}

#[test]
fn the_same_path_as_file_and_dir_is_two_lines() {
    assert_eq!(record_text(&[file("/h/x"), dir("/h/x")]), "file /h/x\ndir /h/x\n");
}

#[test]
fn nothing_is_empty_text() {
    assert_eq!(record_text(&[]), "");
}

#[test]
fn an_entry_whose_path_holds_a_newline_return_or_tab_is_not_written() {
    let entries = [file("/h/ok"), file("/h/a\nb"), dir("/h/c\rd"), file("/h/e\tf"), dir("/h/d")];
    assert_eq!(record_text(&entries), "file /h/ok\ndir /h/d\n");
}

// Unix only: this round trip reads back through parse_record, which keeps absolute paths only, and these are unix
// absolute paths.
#[cfg(unix)]
#[test]
fn an_injected_line_does_not_come_back_from_parse_record() {
    use super::parse_record;

    let injected = [file("/h/a\nfile /victim"), dir("/h/a\rdir /victim"), file("/h/a\tfile /victim")];
    for entry in injected {
        let entries = [file("/h/ok"), entry.clone(), dir("/h/d")];
        assert_eq!(parse_record(&record_text(&entries)), vec![file("/h/ok"), dir("/h/d")], "{entry:?}");
    }
}

#[cfg(unix)]
#[test]
fn an_entry_whose_path_is_not_utf8_is_not_written() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let lossy = PathBuf::from(OsStr::from_bytes(b"/h/\xff"));
    let entries = [file("/h/ok"), RecordEntry::File(lossy.clone()), RecordEntry::Dir(lossy), dir("/h/d")];
    assert_eq!(record_text(&entries), "file /h/ok\ndir /h/d\n");
}
