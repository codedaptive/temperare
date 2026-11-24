//! ZUT for report_warning.rs: a `warnings` entry and an `invalidOptionWarnings` entry, copied from stylelint
//! 18.1.0's report (fixtures/findings.txt, fixtures/invalid_option.txt), read with what each carries.

use super::ReportWarning;

#[test]
fn report_warning_reads_what_an_entry_carries() {
    let rows: [(&str, &str, ReportWarning); 2] = [
        (
            "a warning",
            r##"{"line":1,"column":12,"endLine":1,"endColumn":19,"rule":"color-no-invalid-hex","severity":"error","text":"Invalid hex color \"#fff1az\" (color-no-invalid-hex)"}"##,
            ReportWarning {
                rule: Some("color-no-invalid-hex".to_string()),
                line: Some(1),
                column: Some(12),
                text: "Invalid hex color \"#fff1az\" (color-no-invalid-hex)".to_string(),
            },
        ),
        (
            "an invalid option, text only",
            r#"{"text":"Invalid option value \"medium\" for rule \"color-hex-length\""}"#,
            ReportWarning {
                rule: None,
                line: None,
                column: None,
                text: "Invalid option value \"medium\" for rule \"color-hex-length\"".to_string(),
            },
        ),
    ];
    let failed: Vec<&str> = rows
        .iter()
        .filter(|(_, json, want)| serde_json::from_str::<ReportWarning>(json).ok().as_ref() != Some(want))
        .map(|(name, ..)| *name)
        .collect();
    assert!(failed.is_empty(), "failed rows: {failed:?}");
}
