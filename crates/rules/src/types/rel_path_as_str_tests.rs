//! ZUT for rel_path_as_str.rs: the text of the path as built.

use super::RelPath;

#[test]
fn returns_the_path_text() {
    assert_eq!(RelPath::new("Sources/App/Main.swift").as_str(), "Sources/App/Main.swift");
}
