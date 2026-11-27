//! Unit test for is_markdown_source.rs: `.md` and `.markdown` files are sources; any other extension, or none, is not.

use super::is_markdown_source;
use std::path::Path;

#[test]
fn is_markdown_source_cases() {
    let rows = [
        ("md", "docs/a.md", true),
        ("markdown", "a.markdown", true),
        ("upper case", "README.MD", false),
        ("mdx", "a.mdx", false),
        ("text", "notes.txt", false),
        ("no extension", "md", false),
        ("dot file", ".md", false),
    ];
    for (label, path, expected) in rows {
        assert_eq!(is_markdown_source(Path::new(path)), expected, "{label}");
    }
}
