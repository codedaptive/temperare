//! ZUT for file_report.rs: file results copied from stylelint 18.1.0's report (fixtures/ignored.txt,
//! fixtures/autofixed.txt, fixtures/invalid_option.txt; see fixtures/fixtures.txt), read with what each carries;
//! `ignored` and `autofixed` are false where stylelint leaves them out.

use super::{FileReport, ReportWarning};

fn file(source: &str, ignored: bool, autofixed: bool, invalid: Vec<ReportWarning>) -> FileReport {
    FileReport {
        source: source.to_string(),
        ignored,
        autofixed,
        warnings: vec![],
        invalid_option_warnings: invalid,
        parse_errors: vec![],
    }
}

#[test]
fn file_report_reads_a_files_result() {
    let medium = ReportWarning {
        rule: None,
        line: None,
        column: None,
        text: "Invalid option value \"medium\" for rule \"color-hex-length\"".to_string(),
    };
    let rows: [(&str, &str, FileReport); 3] = [
        (
            "ignored",
            r#"{"source":"/private/tmp/tm-stylelint-fixtures/ign/a.css","deprecations":[],"invalidOptionWarnings":[],"parseErrors":[],"errored":false,"warnings":[],"ignored":true}"#,
            file("/private/tmp/tm-stylelint-fixtures/ign/a.css", true, false, vec![]),
        ),
        (
            "autofixed",
            r#"{"source":"/private/tmp/tm-stylelint-fixtures/fix/a.css","deprecations":[],"invalidOptionWarnings":[],"parseErrors":[],"errored":false,"warnings":[],"autofixed":true}"#,
            file("/private/tmp/tm-stylelint-fixtures/fix/a.css", false, true, vec![]),
        ),
        (
            "an invalid option",
            r#"{"source":"/private/tmp/tm-stylelint-fixtures/inv/a.css","deprecations":[],"invalidOptionWarnings":[{"text":"Invalid option value \"medium\" for rule \"color-hex-length\""}],"parseErrors":[],"errored":true,"warnings":[]}"#,
            file("/private/tmp/tm-stylelint-fixtures/inv/a.css", false, false, vec![medium]),
        ),
    ];
    let failed: Vec<&str> = rows
        .iter()
        .filter(|(_, json, want)| serde_json::from_str::<FileReport>(json).ok().as_ref() != Some(want))
        .map(|(name, ..)| *name)
        .collect();
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
