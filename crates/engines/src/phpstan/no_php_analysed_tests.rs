//! ZUT for no_php_analysed.rs on PHPStan 2.3.7's real stderr with no file left to analyse (fixtures/no_files.txt), its
//! stderr for an unknown level, and hand-built lines.

use super::no_php_analysed;
use procio::Output;

#[test]
fn no_php_analysed_cases() {
    let rows: [(&str, &[u8], bool); 5] = [
        ("captured", include_bytes!("fixtures/no_files.txt"), true),
        ("unknown level", include_bytes!("fixtures/unknown_level.txt"), false),
        ("sentence mid-line", b"No files found to analyse. Try again\n", false),
        ("other text", b" [NOTE] PHPStan runs without PHPStan Turbo.\n", false),
        ("empty", b"", false),
    ];
    for (label, stderr, expected) in rows {
        let out = Output { code: Some(1), stdout: Vec::new(), stderr: stderr.to_vec() };
        assert_eq!(no_php_analysed(&out), expected, "{label}");
    }
}
