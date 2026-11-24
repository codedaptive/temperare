//! ZUT for phpstan_config.rs: the wrapper text for a plain path, written out by hand, and the paths it refuses.

use super::phpstan_config;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

#[test]
fn phpstan_config_cases() {
    let refused = |shown: &str| Err(format!("the phpstan config path {shown} holds ', % or a line break, so tm cannot write it into a config"));
    let rows: [(&str, &Path, Result<String, String>); 6] = [
        ("plain", Path::new("/w/php app/phpstan.neon"),
            Ok("includes:\n    - '/w/php app/phpstan.neon'\nparameters:\n    tmpDir: cache\n    ignoreErrors!: []\n    excludePaths!: {analyse: [], analyseAndScan: []}\n".to_string())),
        ("quote", Path::new("/w/it's/phpstan.neon"), refused("/w/it's/phpstan.neon")),
        ("percent", Path::new("/w/%rootDir%/phpstan.neon"), refused("/w/%rootDir%/phpstan.neon")),
        ("newline", Path::new("/w/a\nb/phpstan.neon"), refused("/w/a\nb/phpstan.neon")),
        ("carriage return", Path::new("/w/a\rb/phpstan.neon"), refused("/w/a\rb/phpstan.neon")),
        ("not UTF-8", Path::new(OsStr::from_bytes(b"/w/\xff/phpstan.neon")),
            Err("the phpstan config path /w/\u{fffd}/phpstan.neon is not UTF-8, so tm cannot write it into a config".to_string())),
    ];
    for (label, config, expected) in rows {
        assert_eq!(phpstan_config(config), expected, "{label}");
    }
}
