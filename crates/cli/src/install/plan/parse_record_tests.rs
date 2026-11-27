//! Unit tests for parse_record.rs: the record's lines back to entries; other lines skipped; repeats once.

use std::path::PathBuf;

use super::{parse_record, record_text};
use crate::RecordEntry;

fn file(path: &str) -> RecordEntry {
    RecordEntry::File(PathBuf::from(path))
}

fn dir(path: &str) -> RecordEntry {
    RecordEntry::Dir(PathBuf::from(path))
}

#[test]
fn reads_files_and_dirs_in_order() {
    let text = "dir /h/.local/bin\nfile /h/.local/bin/cctao\ndir /h/.local/state\n";
    assert_eq!(parse_record(text), vec![dir("/h/.local/bin"), file("/h/.local/bin/cctao"), dir("/h/.local/state")]);
}

#[test]
fn a_path_with_spaces_is_kept_whole() {
    assert_eq!(parse_record("file /h/My Files/SKILL.md\n"), vec![file("/h/My Files/SKILL.md")]);
}

#[test]
fn unknown_empty_and_relative_lines_are_skipped() {
    let text = "\nlnk /h/cctao\nfile\nfile \nfile relative/x\ndir ../up\nFILE /h/a\nfile /h/b\n";
    assert_eq!(parse_record(text), vec![file("/h/b")]);
    assert_eq!(
        parse_record("file /h/cctao\nlink /h/tm\t/h/cctao\ndir /h/bin\n"),
        vec![file("/h/cctao"), dir("/h/bin")]
    );
}

#[test]
fn a_repeat_is_read_once() {
    assert_eq!(parse_record("file /h/a\nfile /h/a\ndir /h/d\nfile /h/a\n"), vec![file("/h/a"), dir("/h/d")]);
}

#[test]
fn a_last_line_without_newline_is_read() {
    assert_eq!(parse_record("dir /h/d"), vec![dir("/h/d")]);
}

#[test]
fn record_text_reads_back_unchanged() {
    let cases = [
        vec![],
        vec![file("/a")],
        vec![dir("/h/.local/state/cctao"), file("/h/.local/state/cctao/installed")],
        vec![file("/x y/z"), dir("/x y"), file("/q")],
    ];
    for entries in cases {
        assert_eq!(parse_record(&record_text(&entries)), entries);
    }
}
