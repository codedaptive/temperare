//! ZUT for config_data.rs: the `-d` text for no base and for base paths whose characters YAML's double-quoted scalar
//! must escape (the escapes for quote, backslash, tab and DEL measured on yamllint 1.38.0, fixtures/fixtures.txt; the
//! others are YAML's own, not measured), and the refusal of a path that is not UTF-8 (on unix only).

use super::config_data;
use std::path::Path;

#[test]
fn config_data_cases() {
    let rows: [(&str, Option<&Path>, &str, Result<&str, &str>); 8] = [
        ("no base", None, "truthy", Ok(r#"{extends: default, ignore: [], rules: {"truthy": {ignore-from-file: []}}}"#)),
        ("plain path", Some(Path::new("/w/top/.yamllint")), "line-length", Ok(r#"{extends: "/w/top/.yamllint", ignore: [], rules: {"line-length": {ignore-from-file: []}}}"#)),
        ("check yes", None, "yes", Ok(r#"{extends: default, ignore: [], rules: {"yes": {ignore-from-file: []}}}"#)),
        ("space", Some(Path::new("/w/my repo/.yamllint")), "truthy", Ok(r#"{extends: "/w/my repo/.yamllint", ignore: [], rules: {"truthy": {ignore-from-file: []}}}"#)),
        ("quote", Some(Path::new("/w/a\"b/.yamllint")), "truthy", Ok(r#"{extends: "/w/a\"b/.yamllint", ignore: [], rules: {"truthy": {ignore-from-file: []}}}"#)),
        ("backslash", Some(Path::new("/w/a\\b/.yamllint")), "truthy", Ok(r#"{extends: "/w/a\\b/.yamllint", ignore: [], rules: {"truthy": {ignore-from-file: []}}}"#)),
        ("tab and newline", Some(Path::new("/w/a\tb\nc/.yamllint")), "truthy", Ok(r#"{extends: "/w/a\x09b\x0Ac/.yamllint", ignore: [], rules: {"truthy": {ignore-from-file: []}}}"#)),
        ("delete and C1", Some(Path::new("/w/a\u{7f}b\u{80}c\u{9f}é/.yamllint")), "truthy", Ok(r#"{extends: "/w/a\x7Fb\x80c\x9Fé/.yamllint", ignore: [], rules: {"truthy": {ignore-from-file: []}}}"#)),
    ];
    for (label, base, check, expected) in rows {
        let expected = expected.map(str::to_string).map_err(str::to_string);
        assert_eq!(config_data(base, check), expected, "{label}");
    }
}

// Unix only: the non-UTF-8 path is built from raw bytes with OsStrExt::from_bytes, which only unix has.
#[cfg(unix)]
#[test]
fn config_data_refuses_a_path_not_utf8() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    let base = Path::new(OsStr::from_bytes(b"/w/bad\xff/.yamllint"));
    let expected = Err("yamllint config path /w/bad\u{fffd}/.yamllint is not valid UTF-8".to_string());
    assert_eq!(config_data(Some(base), "truthy"), expected, "not UTF-8");
}
