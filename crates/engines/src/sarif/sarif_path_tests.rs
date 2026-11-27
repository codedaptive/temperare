//! Unit test for sarif_path.rs: SARIF artifact URIs decoded as Unix or Windows paths.

use super::sarif_path::sarif_path;

#[test]
fn a_file_uri_is_the_path_after_file_slash_slash() {
    assert_eq!(sarif_path("file:///work/project/a.py", false), "/work/project/a.py");
}

#[test]
fn a_file_uri_is_percent_decoded() {
    assert_eq!(sarif_path("file:///work/my%20dir/a%2Eb.py", false), "/work/my dir/a.b.py");
}

#[test]
fn a_relative_uri_is_percent_decoded_and_stays_relative() {
    assert_eq!(sarif_path("src/my%20file.py", false), "src/my file.py");
}

#[test]
fn percent_escapes_are_bytes_read_as_utf8() {
    assert_eq!(sarif_path("caf%C3%A9.py", false), "café.py");
}

#[test]
fn bytes_that_are_not_utf8_become_the_replacement_character() {
    assert_eq!(sarif_path("a%FFb.py", false), "a\u{FFFD}b.py");
}

#[test]
fn lowercase_hex_digits_decode() {
    assert_eq!(sarif_path("a%2fb", false), "a/b");
}

#[test]
fn a_percent_without_two_hex_digits_stays_as_it_is() {
    assert_eq!(sarif_path("100%.py", false), "100%.py");
    assert_eq!(sarif_path("a%2", false), "a%2");
    assert_eq!(sarif_path("a%zz.py", false), "a%zz.py");
    assert_eq!(sarif_path("a%+1.py", false), "a%+1.py");
}

#[test]
fn sarif_paths_follow_the_requested_platform() {
    for (name, uri, windows, expected) in [
        ("drive", "file:///C:/work/a.py", true, r"C:\work\a.py"),
        ("lowercase and escaped space", "file:///c:/my%20dir/a.py", true, r"c:\my dir\a.py"),
        ("escaped colon", "file:///C%3A/x/a.py", true, r"C:\x\a.py"),
        ("parent segment", "file:///C:/a/../b.py", true, r"C:\b.py"),
        ("relative", "src/a.py", true, "src/a.py"),
        ("four slash UNC", "file:////server/share/a.py", true, "//server/share/a.py"),
        ("unix drive URI", "file:///C:/work/a.py", false, "/C:/work/a.py"),
    ] {
        assert_eq!(sarif_path(uri, windows), expected, "{name}");
    }
}
