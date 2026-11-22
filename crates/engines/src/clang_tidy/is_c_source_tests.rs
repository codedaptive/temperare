//! ZUT for is_c_source.rs: the C-family extensions are sources; headers and the rest are not.

use super::is_c_source;
use std::path::Path;

#[test]
fn is_c_source_cases() {
    let rows = [
        ("c", "src/a.c", true),
        ("cc", "a.cc", true),
        ("cpp", "a.cpp", true),
        ("cxx", "a.cxx", true),
        ("c++", "a.c++", true),
        ("m", "a.m", true),
        ("mm", "a.mm", true),
        ("a header", "a.h", false),
        ("a C++ header", "a.hpp", false),
        ("upper case", "a.C", false),
        ("no extension", "c", false),
        ("another language", "a.rs", false),
    ];
    for (label, path, expected) in rows {
        assert_eq!(is_c_source(Path::new(path)), expected, "{label}");
    }
}
