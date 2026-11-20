//! Unit tests for finding.rs: the serialized field names and values (the fields a report gives each finding), and `owner` and `col` left
//! out when None. Serialized with toml (serde_json is not a dependency); the serde names are the same.

use super::{Engine, Finding, Kind, RelPath, Severity};

fn finding(kind: Kind, owner: Option<&str>, col: Option<u32>) -> Finding {
    Finding {
        rule: "no-bool".to_string(),
        engine: Engine::AstGrep,
        kind,
        severity: Severity::Warning,
        owner: owner.map(str::to_string),
        path: RelPath::new("src/a.rs"),
        line: 7,
        col,
        what: "stored Bool".to_string(),
        fix: "Store a bitmap field.".to_string(),
    }
}

#[test]
fn serializes_every_field_by_name() {
    let text = toml::to_string(&finding(Kind::Security, Some("core"), Some(3))).unwrap();
    let expected = "rule = \"no-bool\"\nengine = \"ast-grep\"\nkind = \"security\"\nseverity = \"warning\"\n\
                    owner = \"core\"\npath = \"src/a.rs\"\nline = 7\ncol = 3\nwhat = \"stored Bool\"\n\
                    fix = \"Store a bitmap field.\"\n";
    assert_eq!(text, expected);
}

#[test]
fn a_quality_finding_serializes_its_kind_as_quality() {
    let text = toml::to_string(&finding(Kind::Quality, None, Some(3))).unwrap();
    assert!(text.contains("kind = \"quality\"\n"), "{text}");
}

#[test]
fn owner_is_left_out_when_none() {
    let text = toml::to_string(&finding(Kind::Quality, None, Some(3))).unwrap();
    assert!(!text.contains("owner"), "{text}");
    assert!(text.contains("severity = \"warning\"\npath = \"src/a.rs\"\n"), "{text}");
}

#[test]
fn col_is_left_out_when_none() {
    let text = toml::to_string(&finding(Kind::Quality, None, None)).unwrap();
    assert!(!text.contains("col"), "{text}");
    assert!(text.contains("line = 7\n"), "{text}");
}

#[test]
fn reads_back_without_owner_or_col() {
    let text = "rule = \"r\"\nengine = \"swiftlint\"\nkind = \"quality\"\nseverity = \"error\"\npath = \"a.swift\"\n\
                line = 2\nwhat = \"w\"\nfix = \"f\"\n";
    let read: Finding = toml::from_str(text).unwrap();
    assert_eq!(read.engine, Engine::SwiftLint);
    assert_eq!(read.kind, Kind::Quality);
    assert_eq!(read.severity, Severity::Error);
    assert_eq!(read.owner, None);
    assert_eq!(read.col, None);
}

#[test]
fn reads_back_a_security_finding_with_its_owner() {
    let text = "rule = \"r\"\nengine = \"tessera\"\nkind = \"security\"\nseverity = \"error\"\nowner = \"web\"\n\
                path = \"a.rs\"\nline = 2\nwhat = \"w\"\nfix = \"f\"\n";
    let read: Finding = toml::from_str(text).unwrap();
    assert_eq!(read.kind, Kind::Security);
    assert_eq!(read.owner.as_deref(), Some("web"));
}
