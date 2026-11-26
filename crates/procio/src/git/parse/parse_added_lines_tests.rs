use std::collections::BTreeMap;
use std::ops::Range;
use std::path::PathBuf;

use super::parse_added_lines;

fn files(entries: &[(&[u8], &[Range<u32>])]) -> BTreeMap<PathBuf, Vec<Range<u32>>> {
    entries.iter().map(|(path, ranges)| (PathBuf::from(std::str::from_utf8(path).unwrap()), ranges.to_vec())).collect()
}

#[test]
fn hunk_with_one_added_line_and_no_count() {
    let diff = b"diff --git a.rs a.rs\nindex 1111111..2222222 100644\n--- a.rs\n+++ a.rs\n@@ -3 +3 @@\n-old\n+new\n";
    assert_eq!(parse_added_lines(diff), Some(files(&[(b"a.rs", &[3..4])])));
}

#[test]
fn hunk_with_a_count() {
    let diff = b"diff --git a.rs a.rs\n--- a.rs\n+++ a.rs\n@@ -10,0 +11,3 @@ fn main() {\n+x\n+y\n+z\n";
    assert_eq!(parse_added_lines(diff), Some(files(&[(b"a.rs", &[11..14])])));
}

#[test]
fn hunk_that_only_removes_lines_leaves_the_file_present_and_empty() {
    let diff = b"diff --git a.rs a.rs\n--- a.rs\n+++ a.rs\n@@ -4,2 +3,0 @@\n-gone\n-also gone\n";
    assert_eq!(parse_added_lines(diff), Some(files(&[(b"a.rs", &[])])));
}

#[test]
fn several_hunks_and_several_files() {
    let diff = b"diff --git a.rs a.rs\n--- a.rs\n+++ a.rs\n@@ -1 +1,2 @@\n-a\n+b\n+c\n@@ -9,0 +11 @@\n+d\n\
diff --git src/b.py src/b.py\n--- src/b.py\n+++ src/b.py\n@@ -5,2 +5,2 @@\n-e\n-f\n+g\n+h\n";
    assert_eq!(parse_added_lines(diff), Some(files(&[(b"a.rs", &[1..3, 11..12]), (b"src/b.py", &[5..7])])));
}

#[test]
fn deleted_file_is_absent() {
    let diff = b"diff --git gone.rs gone.rs\ndeleted file mode 100644\nindex 1111111..0000000\n--- gone.rs\n+++ /dev/null\n\
@@ -1,2 +0,0 @@\n-a\n-b\ndiff --git kept.rs kept.rs\n--- kept.rs\n+++ kept.rs\n@@ -0,0 +1 @@\n+k\n";
    assert_eq!(parse_added_lines(diff), Some(files(&[(b"kept.rs", &[1..2])])));
}

#[test]
fn new_file() {
    let diff = b"diff --git new.rs new.rs\nnew file mode 100644\nindex 0000000..3333333\n--- /dev/null\n+++ new.rs\n\
@@ -0,0 +1,3 @@\n+one\n+two\n+three\n";
    assert_eq!(parse_added_lines(diff), Some(files(&[(b"new.rs", &[1..4])])));
}

#[test]
fn path_with_a_space_drops_the_trailing_tab() {
    let diff = b"diff --git my file.rs my file.rs\n--- my file.rs\t\n+++ my file.rs\t\n@@ -0,0 +2 @@\n+x\n";
    assert_eq!(parse_added_lines(diff), Some(files(&[(b"my file.rs", &[2..3])])));
}

#[test]
fn quoted_path_with_octal_escapes() {
    let diff = b"diff --git \"caf\\303\\251.rs\" \"caf\\303\\251.rs\"\n--- \"caf\\303\\251.rs\"\n+++ \"caf\\303\\251.rs\"\n\
@@ -0,0 +1 @@\n+x\n";
    assert_eq!(parse_added_lines(diff), Some(files(&[(b"caf\xC3\xA9.rs", &[1..2])])));
}

#[test]
fn quoted_path_with_quote_and_backslash() {
    let diff = b"--- \"a\\\"b\\\\c.rs\"\n+++ \"a\\\"b\\\\c.rs\"\n@@ -0,0 +4 @@\n+x\n";
    assert_eq!(parse_added_lines(diff), Some(files(&[(b"a\"b\\c.rs", &[4..5])])));
}

#[test]
fn added_line_that_reads_as_a_file_header_is_body() {
    let diff = b"--- notes.md\n+++ notes.md\n@@ -0,0 +1,2 @@\n+++ not a file\n+@@ -1 +1 @@\n";
    assert_eq!(parse_added_lines(diff), Some(files(&[(b"notes.md", &[1..3])])));
}

#[test]
fn added_line_that_reads_as_a_deleted_file_header_is_body() {
    let diff = b"--- a.rs\n+++ a.rs\n@@ -0,0 +1 @@\n+++ /dev/null\n@@ -5,0 +6 @@\n+x\n";
    assert_eq!(parse_added_lines(diff), Some(files(&[(b"a.rs", &[1..2, 6..7])])));
}

#[test]
fn no_newline_markers_do_not_count_as_body_lines() {
    let diff = b"--- a.rs\n+++ a.rs\n@@ -3 +3 @@\n-old\n\\ No newline at end of file\n+new\n\\ No newline at end of file\n\
--- b.rs\n+++ b.rs\n@@ -0,0 +1 @@\n+b\n";
    assert_eq!(parse_added_lines(diff), Some(files(&[(b"a.rs", &[3..4]), (b"b.rs", &[1..2])])));
}

#[test]
fn binary_and_mode_only_files_are_absent() {
    let diff = b"diff --git img.png img.png\nindex 1111111..2222222 100644\nBinary files img.png and img.png differ\n\
diff --git run.sh run.sh\nold mode 100644\nnew mode 100755\n";
    assert_eq!(parse_added_lines(diff), Some(BTreeMap::new()));
}

#[test]
fn empty_output_is_no_files() {
    assert_eq!(parse_added_lines(b""), Some(BTreeMap::new()));
}

#[test]
fn malformed_hunk_header_is_none() {
    assert_eq!(parse_added_lines(b"--- a.rs\n+++ a.rs\n@@ -1 +x @@\n+x\n"), None);
}

#[test]
fn bad_quoted_path_is_none() {
    assert_eq!(parse_added_lines(b"+++ \"a\\q.rs\"\n@@ -0,0 +1 @@\n+x\n"), None);
}

#[test]
fn body_cut_short_is_none() {
    assert_eq!(parse_added_lines(b"--- a.rs\n+++ a.rs\n@@ -0,0 +1,3 @@\n+one\n+two\n"), None);
}

#[test]
fn body_cut_short_without_final_newline_is_none() {
    assert_eq!(parse_added_lines(b"--- a.rs\n+++ a.rs\n@@ -0,0 +1,2 @@\n+one"), None);
}
