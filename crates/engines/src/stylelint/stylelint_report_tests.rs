//! Unit test for stylelint_report.rs: the report found on real stderr (fixtures/report_alone.txt, fixtures/clean.txt with
//! Node's warning lines before it, fixtures/config_error.txt; see fixtures/fixtures.txt), and None where stylelint
//! printed none. The empty and stdout-only rows are hand-built.

use super::{stylelint_report, FileReport};
use procio::Output;

#[test]
fn stylelint_report_reads_the_last_report_line_of_stderr() {
    let alone: &[u8] = include_bytes!("fixtures/report_alone.txt");
    let clean = || {
        Some(vec![FileReport {
            source: "/private/tmp/tm-stylelint-fixtures/clean/a.css".to_string(),
            ignored: false,
            autofixed: false,
            warnings: vec![],
            invalid_option_warnings: vec![],
            parse_errors: vec![],
        }])
    };
    let rows: [(&str, &[u8], &[u8], Option<Vec<FileReport>>); 5] = [
        ("the report alone", b"", alone, clean()),
        ("Node warning lines before it", b"", include_bytes!("fixtures/clean.txt"), clean()),
        ("a configuration error", b"", include_bytes!("fixtures/config_error.txt"), None),
        ("nothing printed", b"", b"", None),
        ("the report on stdout only", alone, b"", None),
    ];
    let failed: Vec<&str> = rows
        .iter()
        .filter(|(_, stdout, stderr, want)| {
            let out = Output { code: Some(0), stdout: stdout.to_vec(), stderr: stderr.to_vec() };
            stylelint_report(&out) != *want
        })
        .map(|(name, ..)| *name)
        .collect();
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
