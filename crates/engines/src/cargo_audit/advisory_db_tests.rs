//! Unit tests for advisory_db.rs: CARGO_HOME first, then HOME's .cargo, an empty value counting as unset.

use super::advisory_db;
use std::ffi::OsStr;
use std::path::PathBuf;

#[test]
fn advisory_db_cases() {
    let rows: [(&str, Option<&str>, Option<&str>, Option<&str>); 6] = [
        ("CARGO_HOME set", Some("/c"), Some("/h"), Some("/c/advisory-db")),
        ("only CARGO_HOME", Some("/c"), None, Some("/c/advisory-db")),
        ("only HOME", None, Some("/h"), Some("/h/.cargo/advisory-db")),
        ("CARGO_HOME empty", Some(""), Some("/h"), Some("/h/.cargo/advisory-db")),
        ("both empty", Some(""), Some(""), None),
        ("neither", None, None, None),
    ];
    for (label, cargo_home, home, expected) in rows {
        assert_eq!(advisory_db(cargo_home.map(OsStr::new), home.map(OsStr::new)), expected.map(PathBuf::from), "{label}");
    }
}
