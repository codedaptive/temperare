use super::parse_hunk_header;

#[test]
fn both_counts_given() {
    assert_eq!(parse_hunk_header(b"@@ -3,2 +5,4 @@"), Some((2, 5, 4)));
}

#[test]
fn omitted_counts_are_one() {
    assert_eq!(parse_hunk_header(b"@@ -7 +9 @@"), Some((1, 9, 1)));
}

#[test]
fn new_count_zero() {
    assert_eq!(parse_hunk_header(b"@@ -4,3 +3,0 @@"), Some((3, 3, 0)));
}

#[test]
fn new_file_hunk() {
    assert_eq!(parse_hunk_header(b"@@ -0,0 +1,3 @@"), Some((0, 1, 3)));
}

#[test]
fn section_text_is_ignored() {
    assert_eq!(parse_hunk_header(b"@@ -10,0 +11,2 @@ fn main() {"), Some((0, 11, 2)));
}

#[test]
fn malformed_headers_are_none() {
    let bad: [&[u8]; 10] = [
        b"@@ -1 +1",
        b"@@ -1 +1 @",
        b"@@ -1 +1 @@x",
        b"@@ 1 +1 @@",
        b"@@ -1 1 @@",
        b"@@ -a +1 @@",
        b"@@ -1 +1,x @@",
        b"@@ -1 ++1 @@",
        b"@@ -1, +1 @@",
        b"@@ -1 +99999999999 @@",
    ];
    for line in bad {
        assert_eq!(parse_hunk_header(line), None, "{}", String::from_utf8_lossy(line));
    }
}
