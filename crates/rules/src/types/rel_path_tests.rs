//! Unit tests for rel_path.rs: a RelPath serializes as its bare text (serde transparent).

use super::RelPath;
use serde::Serialize;

#[derive(Serialize)]
struct Wrap {
    path: RelPath,
}

#[test]
fn serializes_as_plain_text() {
    let text = toml::to_string(&Wrap { path: RelPath::new("src/a.rs") }).unwrap();
    assert_eq!(text, "path = \"src/a.rs\"\n");
}
