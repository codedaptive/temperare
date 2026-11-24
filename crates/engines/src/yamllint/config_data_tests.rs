//! ZUT for config_data.rs: the `-d` text for no base and for base paths whose characters YAML's double-quoted scalar
//! must escape (the escapes for quote, backslash, tab and DEL measured on yamllint 1.38.0, fixtures/fixtures.txt; the
//! others are YAML's own, not measured), and the refusal of a path that is not UTF-8.

use super::config_data;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

#[test]
fn config_data_cases() {
    let not_utf8 = OsStr::from_bytes(b"/w/bad\xff/.yamllint");
    let rows: [(&str, Option<&Path>, &str, Result<&str, &str>); 9] = [
        ("no base", None, "truthy", Ok(r#"{extends: default, ignore: [], rules: {"truthy": {ignore-from-file: []}}}"#)),
        ("plain path", Some(Path::new("/w/top/.yamllint")), "line-length", Ok(r#"{extends: "/w/top/.yamllint", ignore: [], rules: {"line-length": {ignore-from-file: []}}}"#)),
        ("check yes", None, "yes", Ok(r#"{extends: default, ignore: [], rules: {"yes": {ignore-from-file: []}}}"#)),
        ("space", Some(Path::new("/w/my repo/.yamllint")), "truthy", Ok(r#"{extends: "/w/my repo/.yamllint", ignore: [], rules: {"truthy": {ignore-from-file: []}}}"#)),
        ("quote", Some(Path::new("/w/a\"b/.yamllint")), "truthy", Ok(r#"{extends: "/w/a\"b/.yamllint", ignore: [], rules: {"truthy": {ignore-from-file: []}}}"#)),
        ("backslash", Some(Path::new("/w/a\\b/.yamllint")), "truthy", Ok(r#"{extends: "/w/a\\b/.yamllint", ignore: [], rules: {"truthy": {ignore-from-file: []}}}"#)),
        ("tab and newline", Some(Path::new("/w/a\tb\nc/.yamllint")), "truthy", Ok(r#"{extends: "/w/a\x09b\x0Ac/.yamllint", ignore: [], rules: {"truthy": {ignore-from-file: []}}}"#)),
        ("delete and C1", Some(Path::new("/w/a\u{7f}b\u{80}c\u{9f}é/.yamllint")), "truthy", Ok(r#"{extends: "/w/a\x7Fb\x80c\x9Fé/.yamllint", ignore: [], rules: {"truthy": {ignore-from-file: []}}}"#)),
        ("not UTF-8", Some(Path::new(not_utf8)), "truthy", Err("yamllint config path /w/bad\u{fffd}/.yamllint is not valid UTF-8")),
    ];
    for (label, base, check, expected) in rows {
        let expected = expected.map(str::to_string).map_err(str::to_string);
        assert_eq!(config_data(base, check), expected, "{label}");
    }
}
