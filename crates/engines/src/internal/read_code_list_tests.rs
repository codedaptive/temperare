//! Unit test for read_code_list.rs: a full list, a missing source or version line, comments and blanks ignored, no codes.

use crate::code_list::CodeList;
use crate::read_code_list::read_code_list;

#[test]
fn a_full_list_gives_its_source_version_and_codes_in_order() {
    let text = "# source: ShellCheck src\n# version:  0.11.0 \nSC2250\n  SC2086  \nSC1000\n";
    assert_eq!(
        read_code_list(text),
        Ok(CodeList { source: "ShellCheck src", version: "0.11.0", codes: vec!["SC2250", "SC2086", "SC1000"] })
    );
}

#[test]
fn a_list_with_no_source_line_is_an_error() {
    assert_eq!(read_code_list("# version: 1\nSC2086\n"), Err("no \"# source:\" line".to_string()));
}

#[test]
fn a_list_with_no_version_line_is_an_error() {
    assert_eq!(read_code_list("# source: x\nSC2086\n"), Err("no \"# version:\" line".to_string()));
}

#[test]
fn comment_lines_and_blank_lines_are_not_codes() {
    let text = "# a header\n# source: s\n\n   \n# version: v\n# SC2034 is not listed\nSC2086\n  # indented comment\n";
    assert_eq!(read_code_list(text), Ok(CodeList { source: "s", version: "v", codes: vec!["SC2086"] }));
}

#[test]
fn a_list_with_no_codes_is_read_with_none() {
    assert_eq!(read_code_list("# source: s\n# version: pending\n"), Ok(CodeList { source: "s", version: "pending", codes: vec![] }));
}
