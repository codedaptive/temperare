//! ZUT for record_text.rs: one line per entry, in order, each written once; a link line carries its target after
//! a tab; an entry whose path is not UTF-8 or holds a newline, return or tab is left out, so parse_record (pure)
//! reads back no injected entry.

use std::path::PathBuf;

use super::{parse_record, record_text};
use crate::RecordEntry;

fn file(path: &str) -> RecordEntry {
    RecordEntry::File(PathBuf::from(path))
}

fn link(path: &str, target: &str) -> RecordEntry {
    RecordEntry::Link(PathBuf::from(path), PathBuf::from(target))
}

fn dir(path: &str) -> RecordEntry {
    RecordEntry::Dir(PathBuf::from(path))
}

/// What an absolute path starts with before its first `/`: nothing on unix; off unix a path needs a drive to be
/// absolute, and parse_record keeps absolute paths only, so the round trip below names `{DRIVE}/...` paths.
#[cfg(unix)]
const DRIVE: &str = "";
#[cfg(not(unix))]
const DRIVE: &str = "C:";

#[test]
fn files_links_and_dirs_one_per_line_in_order() {
    let entries =
        [dir("/h/.local/bin"), file("/h/.local/bin/temperare"), link("/h/.local/bin/tm", "/h/.local/bin/temperare")];
    assert_eq!(
        record_text(&entries),
        "dir /h/.local/bin\nfile /h/.local/bin/temperare\nlink /h/.local/bin/tm\t/h/.local/bin/temperare\n"
    );
}

#[test]
fn a_repeat_is_written_once_where_it_first_appears() {
    let entries = [file("/h/a"), dir("/h/d"), link("/h/l", "/t"), file("/h/a"), link("/h/l", "/t"), file("/h/b")];
    assert_eq!(record_text(&entries), "file /h/a\ndir /h/d\nlink /h/l\t/t\nfile /h/b\n");
}

#[test]
fn the_same_link_with_another_target_is_another_line() {
    assert_eq!(record_text(&[link("/h/tm", "/a"), link("/h/tm", "/b")]), "link /h/tm\t/a\nlink /h/tm\t/b\n");
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
    let entries = [
        file("/h/ok"),
        file("/h/a\nb"),
        dir("/h/c\rd"),
        file("/h/e\tf"),
        link("/h/tm\nx", "/t"),
        link("/h/tm", "/t\tx"),
        dir("/h/d"),
    ];
    assert_eq!(record_text(&entries), "file /h/ok\ndir /h/d\n");
}

#[test]
fn an_injected_line_does_not_come_back_from_parse_record() {
    let injected = [
        file(&format!("{DRIVE}/h/a\nfile {DRIVE}/victim")),
        dir(&format!("{DRIVE}/h/a\rdir {DRIVE}/victim")),
        link(&format!("{DRIVE}/h/tm"), &format!("{DRIVE}/t\nfile {DRIVE}/victim")),
        link(&format!("{DRIVE}/h/tm\nfile {DRIVE}/victim"), &format!("{DRIVE}/t")),
    ];
    let (ok, d) = (file(&format!("{DRIVE}/h/ok")), dir(&format!("{DRIVE}/h/d")));
    for entry in injected {
        let entries = [ok.clone(), entry.clone(), d.clone()];
        assert_eq!(parse_record(&record_text(&entries)), vec![ok.clone(), d.clone()], "{entry:?}");
    }
}

#[cfg(unix)]
#[test]
fn an_entry_whose_path_is_not_utf8_is_not_written() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let lossy = PathBuf::from(OsStr::from_bytes(b"/h/\xff"));
    let entries = [
        file("/h/ok"),
        RecordEntry::File(lossy.clone()),
        RecordEntry::Dir(lossy.clone()),
        RecordEntry::Link(lossy.clone(), PathBuf::from("/t")),
        RecordEntry::Link(PathBuf::from("/h/tm"), lossy),
        dir("/h/d"),
    ];
    assert_eq!(record_text(&entries), "file /h/ok\ndir /h/d\n");
}
