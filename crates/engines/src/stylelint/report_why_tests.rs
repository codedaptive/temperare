//! Unit test for report_why.rs: the first file result of real stylelint 18.1.0 stderr (fixtures/*.txt; see
//! fixtures/fixtures.txt), each capture's folder under `/private/tmp/tm-stylelint-fixtures` passed as `cwd` and
//! `folder`; no folder is read. The `parseErrors` row is hand-built: no measured run filled `parseErrors`.

use super::{report_why, stylelint_report, FileReport, ReportWarning};
use procio::Output;
use std::path::Path;

const ROOT: &str = "/private/tmp/tm-stylelint-fixtures";

fn first(stderr: &[u8]) -> FileReport {
    let out = Output { code: Some(2), stdout: Vec::new(), stderr: stderr.to_vec() };
    stylelint_report(&out).and_then(|files| files.into_iter().next()).expect("a fixture with a report")
}

#[test]
fn report_why_says_why_a_result_is_no_answer() {
    let ignored = include_bytes!("fixtures/ignored.txt");
    let unparsed = FileReport {
        source: format!("{ROOT}/pe/a.css"),
        ignored: false,
        autofixed: false,
        warnings: vec![],
        invalid_option_warnings: vec![],
        parse_errors: vec![ReportWarning { rule: None, line: None, column: None, text: "Unknown word".to_string() }],
    };
    let rows: [(&str, FileReport, &str, &str, Option<&str>); 8] = [
        ("ignored", first(ignored), "ign", "ign", Some("stylelint ignored a.css through ignoreFiles in its config; exceptions go in the rule file")),
        ("ignored outside the folder", first(ignored), "ign", "ign/sub", Some("stylelint ignored /private/tmp/tm-stylelint-fixtures/ign/a.css through ignoreFiles in its config; exceptions go in the rule file")),
        ("autofixed", first(include_bytes!("fixtures/autofixed.txt")), "fix", "fix", Some("stylelint rewrote a.css: its config turns on fix, and cctao never edits the code it checks")),
        ("a CSS syntax error", first(include_bytes!("fixtures/syntax_error.txt")), "syn", "syn", Some("stylelint could not parse broken.css: Unclosed block (CssSyntaxError)")),
        ("a parseErrors entry", unparsed, "pe", "pe", Some("stylelint could not parse a.css: Unknown word")),
        ("an invalid option", first(include_bytes!("fixtures/invalid_option.txt")), "inv", "inv", Some("stylelint's config is invalid for a.css: Invalid option value \"medium\" for rule \"color-hex-length\"")),
        ("findings", first(include_bytes!("fixtures/findings.txt")), "two", "two", None),
        ("clean", first(include_bytes!("fixtures/clean.txt")), "clean", "clean", None),
    ];
    let failed: Vec<&str> = rows
        .iter()
        .filter(|(_, file, cwd, folder, want)| {
            let (cwd, folder) = (format!("{ROOT}/{cwd}"), format!("{ROOT}/{folder}"));
            report_why(file, Path::new(&cwd), Path::new(&folder)).as_deref() != *want
        })
        .map(|(name, ..)| *name)
        .collect();
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
