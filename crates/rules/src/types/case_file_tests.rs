//! Unit tests for case_file.rs: a CaseFile equals another only when both its path and its text match.

use super::CaseFile;

fn file(path: &str, text: &'static str) -> CaseFile {
    CaseFile { path: path.to_string(), text }
}

#[test]
fn a_clone_equals_the_original() {
    let original = file("src/lib.rs", "fn f() {}\n");
    assert_eq!(original.clone(), CaseFile { path: "src/lib.rs".to_string(), text: "fn f() {}\n" });
}

#[test]
fn a_different_path_is_a_different_file() {
    assert_ne!(file("src/lib.rs", "fn f() {}\n"), file("src/main.rs", "fn f() {}\n"));
}

#[test]
fn a_different_text_is_a_different_file() {
    assert_ne!(file("src/lib.rs", "fn f() {}\n"), file("src/lib.rs", "fn g() {}\n"));
}
