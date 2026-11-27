//! Unit tests for unfit_for_record.rs: a path that is not UTF-8, or holds a newline, carriage return or tab anywhere, is
//! unfit; a plain path, spaces and other control characters are not.

use std::path::Path;

use super::unfit_for_record::unfit_for_record;

#[test]
fn a_newline_is_unfit() {
    assert!(unfit_for_record(Path::new("/h/a\nfile /victim")));
}

#[test]
fn a_carriage_return_is_unfit() {
    assert!(unfit_for_record(Path::new("/h/a\r")));
}

#[test]
fn a_tab_is_unfit() {
    assert!(unfit_for_record(Path::new("\t/h/a")));
}

#[test]
fn a_plain_path_is_fit() {
    assert!(!unfit_for_record(Path::new("/h/.local/bin/cctao")));
}

#[test]
fn a_space_or_other_control_character_is_fit() {
    assert!(!unfit_for_record(Path::new("/h/my bin/te\u{1}mp\u{b}erare")));
}

#[test]
fn a_non_ascii_utf8_path_is_fit() {
    assert!(!unfit_for_record(Path::new("/h/caf\u{e9}/cctao")));
}

#[cfg(unix)]
#[test]
fn a_path_that_is_not_utf8_is_unfit() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    assert!(unfit_for_record(Path::new(OsStr::from_bytes(b"/h/\xff"))));
}
