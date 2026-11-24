//! ZUT for is_dart_source.rs: a `.dart` file is a source; other extensions, upper case and no extension are not.

use super::is_dart_source;
use std::path::Path;

#[test]
fn is_dart_source_cases() {
    let rows = [
        ("dart", "lib/a.dart", true),
        ("a pipe in the name", "lib/pipe|name.dart", true),
        ("upper case", "a.DART", false),
        ("another extension after", "a.dart.txt", false),
        ("a longer extension", "a.darts", false),
        ("no extension", "dart", false),
        ("another language", "a.rs", false),
    ];
    for (label, path, expected) in rows {
        assert_eq!(is_dart_source(Path::new(path)), expected, "{label}");
    }
}
