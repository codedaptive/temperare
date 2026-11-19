use std::{fs, path::Path};

use engines::{availability, run_rules, swiftlint::{parse_swiftlint, swiftlint_cmd}, Availability};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

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
fn swiftlint_finding_in_a_private_tmp_repo_is_kept() {
    assert_eq!(availability(Engine::SwiftLint), Availability::Available);
    let folder = Path::new("/tmp").join(format!("engines-zit-{}-swiftlint-private", std::process::id()));
    fs::create_dir(&folder).unwrap();
    let folder = fs::canonicalize(folder).unwrap();
    fs::create_dir(folder.join("Sources")).unwrap();
    fs::write(folder.join("Sources/Cast.swift"), "func example(_ value: Any) {\n    let text = value as! String\n    print(text)\n}\n").unwrap();
    let result = procio::run(&swiftlint_cmd("force_cast", None, &folder))
        .map(|out| parse_swiftlint(&out, "force_cast", &folder, &folder));
    fs::remove_dir_all(&folder).unwrap();
    #[cfg(target_os = "macos")]
    assert!(folder.starts_with("/private/tmp"), "{folder:?}");
    let hits = result.unwrap().unwrap();
    assert_eq!(hits.len(), 1, "{hits:?}");
    assert_eq!(hits[0].path, RelPath::new("Sources/Cast.swift"));
    assert_eq!(hits[0].line, 2);
}

#[test]
fn swiftlint_unknown_rule_is_failed_naming_the_identifier() {
    assert_eq!(availability(Engine::SwiftLint), Availability::Available);
    let folder = std::env::temp_dir().join(format!("engines-zit-{}-swiftlint-unknown", std::process::id()));
    fs::create_dir(&folder).unwrap();
    let folder = fs::canonicalize(folder).unwrap();
    fs::write(folder.join("Example.swift"), "let value = 1\n").unwrap();
    fs::write(folder.join("temperare.toml"), "version = 1\n[[rule]]\nid = \"unknown-swift-rule\"\nengine = \"swiftlint\"\ncheck = \"no_such_rule\"\nfix = \"Name a known rule.\"\n").unwrap();
    let file = RuleFile::read(&folder.join("temperare.toml")).unwrap();
    let outcomes = run_rules(&file, &folder, &[]);
    fs::remove_dir_all(&folder).unwrap();
    assert_eq!(outcomes, vec![("unknown-swift-rule".into(), RuleOutcome::Failed {
        rule: "unknown-swift-rule".into(),
        engine: Engine::SwiftLint,
        code: Some(0),
        stderr: "swiftlint check `no_such_rule` is not a rule this swiftlint knows (warning: 'no_such_rule' is not a valid rule identifier): tm cannot report it; name a rule `swiftlint rules` lists, or a custom rule the rule's config defines".to_string(),
    })]);
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
