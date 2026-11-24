//! ZUT for unreported_files.rs: the files handed to sqlfluff 4.5.0 that its report leaves out, named with its stderr;
//! None when all are reported or there is no report. fixtures/large_file.txt is the stderr of a run whose stdout was
//! `[]` (fixtures/fixtures.txt).

use super::unreported_files;
use procio::Output;
use std::path::PathBuf;

#[test]
fn unreported_files_cases() {
    let findings: &[u8] = include_bytes!("fixtures/findings.json");
    let large = "sqlfluff did not check big.sql; it said: WARNING    Length of file 'big.sql' is 25500 bytes which is over the limit of 20000 bytes. Skipping to avoid parser lock. Users can increase this limit in their config by setting the 'large_file_skip_byte_limit' value, or disable by setting it to zero.";
    let rows: [(&str, &[u8], &[u8], &[&str], Option<&str>); 5] = [
        ("the large-file run", b"[]\n", include_bytes!("fixtures/large_file.txt"), &["big.sql"], Some(large)),
        ("every file reported", findings, b"", &["a.sql", "clean.sql", "sub/b.sql"], None),
        ("two of four left out", findings, b"WARNING    skipped\n", &["a.sql", "x.sql", "sub/b.sql", "y.sql"], Some("sqlfluff did not check x.sql, y.sql; it said: WARNING    skipped")),
        ("a file in another folder", findings, b"", &["b.sql"], Some("sqlfluff did not check b.sql; it said: ")),
        ("no JSON report", b"", b"KeyError: 'Unknown dialect'\n", &["a.sql"], None),
    ];
    for (label, stdout, stderr, files, expected) in rows {
        let out = Output { code: Some(0), stdout: stdout.to_vec(), stderr: stderr.to_vec() };
        let files: Vec<PathBuf> = files.iter().map(PathBuf::from).collect();
        assert_eq!(unreported_files(&out, &files).as_deref(), expected, "{label}");
    }
}
