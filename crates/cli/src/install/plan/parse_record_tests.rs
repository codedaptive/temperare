//! ZUT for parse_record.rs: the record's lines back to entries; other lines skipped; repeats once.

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

#[test]
fn reads_files_links_and_dirs_in_order() {
    let text = "dir /h/.local/bin\nfile /h/.local/bin/temperare\nlink /h/.local/bin/tm\t/h/.local/bin/temperare\n";
    assert_eq!(
        parse_record(text),
        vec![dir("/h/.local/bin"), file("/h/.local/bin/temperare"), link("/h/.local/bin/tm", "/h/.local/bin/temperare")]
    );
}

#[test]
fn a_path_with_spaces_is_kept_whole() {
    assert_eq!(parse_record("file /h/My Files/SKILL.md\n"), vec![file("/h/My Files/SKILL.md")]);
    assert_eq!(
        parse_record("link /h/my bin/tm\t/opt/my apps/temperare\n"),
        vec![link("/h/my bin/tm", "/opt/my apps/temperare")]
    );
}

#[test]
fn a_link_splits_at_the_first_tab() {
    assert_eq!(parse_record("link /h/tm\t/t/a\tb\n"), vec![link("/h/tm", "/t/a\tb")]);
}

#[test]
fn unknown_empty_and_relative_lines_are_skipped() {
    let text = "\nlnk /h/tm\nfile\nfile \nfile relative/x\ndir ../up\nFILE /h/a\nfile /h/b\n";
    assert_eq!(parse_record(text), vec![file("/h/b")]);
}

#[test]
fn a_link_without_a_tab_or_with_a_relative_path_is_skipped() {
    let text = "link /h/tm /h/temperare\nlink /h/tm\nlink tm\t/h/temperare\nlink /h/tm\ttemperare\nlink /h/tm\t\n\
                link /h/ok\t/t\n";
    assert_eq!(parse_record(text), vec![link("/h/ok", "/t")]);
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
        vec![dir("/h/.local/state/temperare"), file("/h/.local/state/temperare/installed")],
        vec![file("/x y/z"), dir("/x y"), link("/x y/tm", "/q r/temperare"), file("/q")],
    ];
    for entries in cases {
        assert_eq!(parse_record(&record_text(&entries)), entries);
    }
}
