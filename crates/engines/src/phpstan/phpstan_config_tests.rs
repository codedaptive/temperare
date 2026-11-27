//! ZUT for phpstan_config.rs: the wrapper text for a plain path, written out by hand, and the paths it refuses (the
//! not-UTF-8 path on unix only).

use super::phpstan_config;
use std::path::Path;

#[test]
fn phpstan_config_cases() {
    let refused = |shown: &str| Err(format!("the phpstan config path {shown} holds ', % or a line break, so cctao cannot write it into a config"));
    let rows: [(&str, &Path, Result<String, String>); 5] = [
        ("plain", Path::new("/w/php app/phpstan.neon"),
            Ok("includes:\n    - '/w/php app/phpstan.neon'\nparameters:\n    tmpDir: cache\n    ignoreErrors!: []\n    excludePaths!: {analyse: [], analyseAndScan: []}\n".to_string())),
        ("quote", Path::new("/w/it's/phpstan.neon"), refused("/w/it's/phpstan.neon")),
        ("percent", Path::new("/w/%rootDir%/phpstan.neon"), refused("/w/%rootDir%/phpstan.neon")),
        ("newline", Path::new("/w/a\nb/phpstan.neon"), refused("/w/a\nb/phpstan.neon")),
        ("carriage return", Path::new("/w/a\rb/phpstan.neon"), refused("/w/a\rb/phpstan.neon")),
    ];
    for (label, config, expected) in rows {
        assert_eq!(phpstan_config(config), expected, "{label}");
    }
}

// Unix only: the non-UTF-8 path is built from raw bytes with OsStrExt::from_bytes, which only unix has.
#[cfg(unix)]
#[test]
fn phpstan_config_refuses_a_path_not_utf8() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;
    let expected = Err("the phpstan config path /w/\u{fffd}/phpstan.neon is not UTF-8, so cctao cannot write it into a config".to_string());
    assert_eq!(phpstan_config(Path::new(OsStr::from_bytes(b"/w/\xff/phpstan.neon"))), expected, "not UTF-8");
}
