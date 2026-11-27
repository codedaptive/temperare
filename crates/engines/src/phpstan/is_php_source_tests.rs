//! Unit test for is_php_source.rs: a `.php` file is a source; any other extension, or none, is not.

use super::is_php_source;
use std::path::Path;

#[test]
fn is_php_source_cases() {
    let rows = [
        ("php", "src/a.php", true),
        ("upper case", "a.PHP", false),
        ("phtml", "a.phtml", false),
        ("a stub", "a.stub", false),
        ("no extension", "php", false),
        ("another language", "a.rs", false),
    ];
    for (label, path, expected) in rows {
        assert_eq!(is_php_source(Path::new(path)), expected, "{label}");
    }
}
