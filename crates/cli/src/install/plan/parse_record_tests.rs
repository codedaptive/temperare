//! ZUT for parse_record.rs: the record's lines back to entries; other lines skipped; repeats once.

use std::path::PathBuf;

use super::{parse_record, record_text};
use crate::RecordEntry;

/// What an absolute path starts with before its first `/`: nothing on unix; off unix a path needs a drive to be
/// absolute, and parse_record keeps absolute paths only. Every `{DRIVE}/...` below reads `/...` on unix.
#[cfg(unix)]
const DRIVE: &str = "";
#[cfg(not(unix))]
const DRIVE: &str = "C:";

/// `path` (written from its first `/`) as an absolute path.
fn abs(path: &str) -> PathBuf {
    PathBuf::from(format!("{DRIVE}{path}"))
}

fn file(path: &str) -> RecordEntry {
    RecordEntry::File(abs(path))
}

fn link(path: &str, target: &str) -> RecordEntry {
    RecordEntry::Link(abs(path), abs(target))
}

fn dir(path: &str) -> RecordEntry {
    RecordEntry::Dir(abs(path))
}

#[test]
fn reads_files_links_and_dirs_in_order() {
    let text = format!(
        "dir {DRIVE}/h/.local/bin\nfile {DRIVE}/h/.local/bin/temperare\n\
         link {DRIVE}/h/.local/bin/tm\t{DRIVE}/h/.local/bin/temperare\n"
    );
    assert_eq!(
        parse_record(&text),
        vec![dir("/h/.local/bin"), file("/h/.local/bin/temperare"), link("/h/.local/bin/tm", "/h/.local/bin/temperare")]
    );
}

#[test]
fn a_path_with_spaces_is_kept_whole() {
    assert_eq!(parse_record(&format!("file {DRIVE}/h/My Files/SKILL.md\n")), vec![file("/h/My Files/SKILL.md")]);
    assert_eq!(
        parse_record(&format!("link {DRIVE}/h/my bin/tm\t{DRIVE}/opt/my apps/temperare\n")),
        vec![link("/h/my bin/tm", "/opt/my apps/temperare")]
    );
}

#[test]
fn a_link_splits_at_the_first_tab() {
    assert_eq!(parse_record(&format!("link {DRIVE}/h/tm\t{DRIVE}/t/a\tb\n")), vec![link("/h/tm", "/t/a\tb")]);
}

#[test]
fn unknown_empty_and_relative_lines_are_skipped() {
    let text =
        format!("\nlnk {DRIVE}/h/tm\nfile\nfile \nfile relative/x\ndir ../up\nFILE {DRIVE}/h/a\nfile {DRIVE}/h/b\n");
    assert_eq!(parse_record(&text), vec![file("/h/b")]);
}

#[test]
fn a_link_without_a_tab_or_with_a_relative_path_is_skipped() {
    let text = format!(
        "link {DRIVE}/h/tm {DRIVE}/h/temperare\nlink {DRIVE}/h/tm\nlink tm\t{DRIVE}/h/temperare\n\
         link {DRIVE}/h/tm\ttemperare\nlink {DRIVE}/h/tm\t\nlink {DRIVE}/h/ok\t{DRIVE}/t\n"
    );
    assert_eq!(parse_record(&text), vec![link("/h/ok", "/t")]);
}

#[test]
fn a_repeat_is_read_once() {
    let text = format!("file {DRIVE}/h/a\nfile {DRIVE}/h/a\ndir {DRIVE}/h/d\nfile {DRIVE}/h/a\n");
    assert_eq!(parse_record(&text), vec![file("/h/a"), dir("/h/d")]);
}

#[test]
fn a_last_line_without_newline_is_read() {
    assert_eq!(parse_record(&format!("dir {DRIVE}/h/d")), vec![dir("/h/d")]);
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
