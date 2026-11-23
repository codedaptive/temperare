//! ZUT for is_css_source.rs: a `.css` file in any ASCII case is a source; any other extension, or none, is not.

use super::is_css_source;
use std::path::Path;

#[test]
fn is_css_source_cases() {
    let rows = [
        ("css", "src/a.css", true),
        ("upper case", "a.CSS", true),
        ("mixed case", "a.Css", true),
        ("scss", "a.scss", false),
        ("less", "a.less", false),
        ("a source map", "a.css.map", false),
        ("no extension", "css", false),
        ("html", "a.html", false),
    ];
    for (label, path, expected) in rows {
        assert_eq!(is_css_source(Path::new(path)), expected, "{label}");
    }
}
