use std::ffi::OsStr;

use crate::has_path_separator::has_path_separator;

#[test]
fn a_separator_anywhere_makes_a_path() {
    let rows = [("git", false), ("./git", true), ("/usr/bin/git", true), ("a/b", true), ("", false)];
    for (program, expected) in rows {
        assert_eq!(has_path_separator(OsStr::new(program)), expected, "{program:?}");
    }
}
