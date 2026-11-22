//! ZUT for no_compile_database.rs: the why read from clang-tidy's stderr (no_database.txt is the real stderr of a run
//! with no database; fixtures/commands.txt), the texts written out in full.

use super::no_compile_database;
use procio::Output;

#[test]
fn no_compile_database_cases() {
    let why = |file: &str| {
        Some(format!("no compile database (compile_commands.json) or compile flags (compile_flags.txt) for {file} or any folder above it, so clang-tidy would check it without flags"))
    };
    let rows: [(&str, &[u8], Option<String>); 4] = [
        ("the real stderr", include_bytes!("fixtures/no_database.txt"), why("/private/tmp/clang-tidy-fixtures.H87jA4/nodb/a.c")),
        ("no file line", b"Error while trying to load a compilation database:\nRunning without flags.\n", why("a file it was given")),
        ("the file line alone", b"Could not auto-detect compilation database for file \"/w/a.c\"\n", None),
        ("empty stderr", b"", None),
    ];
    for (label, stderr, expected) in rows {
        let out = Output { code: Some(0), stdout: Vec::new(), stderr: stderr.to_vec() };
        assert_eq!(no_compile_database(&out), expected, "{label}");
    }
}
