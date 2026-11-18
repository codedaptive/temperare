use std::fs;

use engines::{availability, swiftlint::{parse_swiftlint, swiftlint_cmd}, Availability};
use rules::{Engine, RelPath};

#[test]
fn swiftlint_reports_force_cast_but_not_other_rules_or_clean_lines() {
    assert_eq!(availability(Engine::SwiftLint), Availability::Available);
    let folder = std::env::temp_dir().join(format!("engines-zit-{}-swiftlint", std::process::id()));
    fs::create_dir(&folder).unwrap();
    let folder = fs::canonicalize(folder).unwrap();
    fs::write(folder.join("Example.swift"), "func example(_ value: Any) {  \n    let text = value as! String\n    print(text)\n}\n").unwrap();
    let result = procio::run(&swiftlint_cmd("force_cast", None, &folder))
        .map(|out| parse_swiftlint(&out, "force_cast", &folder, &folder));
    fs::remove_dir_all(&folder).unwrap();
    let hits = result.unwrap().unwrap();
    assert_eq!(hits.len(), 1, "{hits:?}");
    assert_eq!(hits[0].path, RelPath::new("Example.swift"));
    assert_eq!(hits[0].line, 2);
    assert!(hits[0].what.to_lowercase().contains("cast"), "{:?}", hits[0]);
}

#[test]
fn d1_swiftlint_comma_separated_identifier_is_an_error() {
    assert_eq!(availability(Engine::SwiftLint), Availability::Available);
    let folder = std::env::temp_dir().join(format!("engines-zit-{}-swiftlint-invalid", std::process::id()));
    fs::create_dir(&folder).unwrap();
    let folder = fs::canonicalize(folder).unwrap();
    fs::write(folder.join("Example.swift"), "let value = 1\n").unwrap();
    let result = procio::run(&swiftlint_cmd("force_cast,force_try", None, &folder))
        .map(|out| parse_swiftlint(&out, "force_cast,force_try", &folder, &folder));
    fs::remove_dir_all(&folder).unwrap();
    let got = result.unwrap();
    assert!(got.is_err(), "{got:?}");
    assert!(got.unwrap_err().why.contains("is not a valid rule identifier"));
}
