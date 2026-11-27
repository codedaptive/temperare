//! Unit test for inspected_files.rs: the file list of real rubocop reports (see fixtures/fixtures.txt), and output that is
//! not a report.

use super::inspected_files;
use rules::RelPath;

#[test]
fn inspected_files_lists_every_file_of_the_report_in_order() {
    let cases: [(&str, &[u8], Vec<&str>); 5] = [
        ("offenses", include_bytes!("fixtures/check_offenses.json"), vec!["example.rb", "lib/other.rb"]),
        ("clean", include_bytes!("fixtures/check_clean.json"), vec!["example.rb"]),
        ("syntax", include_bytes!("fixtures/check_syntax.json"), vec!["broken.rb"]),
        ("unknown cop", include_bytes!("fixtures/check_unknown.json"), vec![]),
        ("not json", b"garbage", vec![]),
    ];
    let mut failed = Vec::new();
    for (name, stdout, want) in cases {
        let result = std::panic::catch_unwind(|| {
            let want: Vec<RelPath> = want.iter().map(|path| RelPath::new(path)).collect();
            assert_eq!(inspected_files(stdout), want, "{name}");
        });
        if result.is_err() { failed.push(name); }
    }
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
