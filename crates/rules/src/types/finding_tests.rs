//! ZUT for finding.rs: the serialized field names and values (codeteam's finding fields), and `col` left out when
//! None. Serialized with toml (serde_json is not a dependency); the serde names are the same.

use super::{Engine, Finding, RelPath, Severity};

fn finding(col: Option<u32>) -> Finding {
    Finding {
        rule: "no-bool".to_string(),
        engine: Engine::AstGrep,
        kind: "rule".to_string(),
        severity: Severity::Warning,
        path: RelPath::new("src/a.rs"),
        line: 7,
        col,
        what: "stored Bool".to_string(),
        fix: "Store a bitmap field.".to_string(),
    }
}

#[test]
fn serializes_every_field_by_name() {
    let text = toml::to_string(&finding(Some(3))).unwrap();
    let expected = "rule = \"no-bool\"\nengine = \"ast-grep\"\nkind = \"rule\"\nseverity = \"warning\"\n\
                    path = \"src/a.rs\"\nline = 7\ncol = 3\nwhat = \"stored Bool\"\nfix = \"Store a bitmap field.\"\n";
    assert_eq!(text, expected);
}

#[test]
fn col_is_left_out_when_none() {
    let text = toml::to_string(&finding(None)).unwrap();
    assert!(!text.contains("col"), "{text}");
    assert!(text.contains("line = 7\n"), "{text}");
}

#[test]
fn reads_back_without_col() {
    let text = "rule = \"r\"\nengine = \"swiftlint\"\nkind = \"rule\"\nseverity = \"error\"\npath = \"a.swift\"\n\
                line = 2\nwhat = \"w\"\nfix = \"f\"\n";
    let read: Finding = toml::from_str(text).unwrap();
    assert_eq!(read.engine, Engine::SwiftLint);
    assert_eq!(read.severity, Severity::Error);
    assert_eq!(read.col, None);
}
