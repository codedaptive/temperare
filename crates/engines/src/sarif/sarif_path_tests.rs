//! ZUT for sarif_path.rs: SARIF artifact URIs, read as paths.

use super::sarif_path::sarif_path;

#[test]
fn a_file_uri_is_the_path_after_file_slash_slash() {
    assert_eq!(sarif_path("file:///work/project/a.py"), "/work/project/a.py");
}

#[test]
fn a_file_uri_is_percent_decoded() {
    assert_eq!(sarif_path("file:///work/my%20dir/a%2Eb.py"), "/work/my dir/a.b.py");
}

#[test]
fn a_relative_uri_is_percent_decoded_and_stays_relative() {
    assert_eq!(sarif_path("src/my%20file.py"), "src/my file.py");
}

#[test]
fn percent_escapes_are_bytes_read_as_utf8() {
    assert_eq!(sarif_path("caf%C3%A9.py"), "café.py");
}

#[test]
fn bytes_that_are_not_utf8_become_the_replacement_character() {
    assert_eq!(sarif_path("a%FFb.py"), "a\u{FFFD}b.py");
}

#[test]
fn lowercase_hex_digits_decode() {
    assert_eq!(sarif_path("a%2fb"), "a/b");
}

#[test]
fn a_percent_without_two_hex_digits_stays_as_it_is() {
    assert_eq!(sarif_path("100%.py"), "100%.py");
    assert_eq!(sarif_path("a%2"), "a%2");
    assert_eq!(sarif_path("a%zz.py"), "a%zz.py");
    assert_eq!(sarif_path("a%+1.py"), "a%+1.py");
}
