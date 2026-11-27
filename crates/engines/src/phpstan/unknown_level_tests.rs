//! Unit test for unknown_level.rs on PHPStan 2.3.7's real stderr for `--level=11` (fixtures/unknown_level.txt), and on
//! stderr that names another level or says nothing of levels.

use super::unknown_level;
use procio::Output;

const SAID: &str =
    "Level config file phar:///opt/homebrew/Cellar/phpstan/2.3.7/bin/phpstan/conf/config.level11.neon was not found.";

#[test]
fn unknown_level_cases() {
    let captured: &[u8] = include_bytes!("fixtures/unknown_level.txt");
    let indented = format!("  {SAID}  \n");
    let rows: [(&str, &[u8], &str, Option<&str>); 6] = [
        ("captured, its level", captured, "11", Some(SAID)),
        ("captured, another level", captured, "12", None),
        ("captured, a level ending in its digits", captured, "1", None),
        ("indented line", indented.as_bytes(), "11", Some(SAID)),
        ("no such line", include_bytes!("fixtures/no_files.txt"), "11", None),
        ("empty", b"", "11", None),
    ];
    for (label, stderr, check, expected) in rows {
        let out = Output { code: Some(1), stdout: Vec::new(), stderr: stderr.to_vec() };
        assert_eq!(unknown_level(&out, check).as_deref(), expected, "{label}");
    }
}
