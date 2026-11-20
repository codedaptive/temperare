use std::{ffi::OsStr, fs, path::Path};

use engines::{availability, entry_cmd, run_rules_counted, swiftlint::{parse_swiftlint, SWIFTLINT}, Availability};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};

#[test]
fn swiftlint_reports_force_cast_but_not_other_rules_or_clean_lines() {
    assert_eq!(availability(Engine::SwiftLint), Availability::Available);
    let folder = std::env::temp_dir().join(format!("engines-zit-{}-swiftlint", std::process::id()));
    fs::create_dir(&folder).unwrap();
    let folder = fs::canonicalize(folder).unwrap();
    fs::write(folder.join("Example.swift"), "func example(_ value: Any) {  \n    let text = value as! String\n    print(text)\n}\n").unwrap();
    let result = procio::run(&entry_cmd(&SWIFTLINT, OsStr::new("force_cast"), &[], None, &folder))
        .map(|out| parse_swiftlint(&out, "force_cast", &folder, &folder));
    fs::remove_dir_all(&folder).unwrap();
    let hits = result.unwrap().unwrap();
    assert_eq!(hits.len(), 1, "{hits:?}");
    assert_eq!(hits[0].path, RelPath::new("Example.swift"));
    assert_eq!(hits[0].line, Some(2));
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
    let result = procio::run(&entry_cmd(&SWIFTLINT, OsStr::new("force_cast"), &[], None, &folder))
        .map(|out| parse_swiftlint(&out, "force_cast", &folder, &folder));
    fs::remove_dir_all(&folder).unwrap();
    #[cfg(target_os = "macos")]
    assert!(folder.starts_with("/private/tmp"), "{folder:?}");
    let hits = result.unwrap().unwrap();
    assert_eq!(hits.len(), 1, "{hits:?}");
    assert_eq!(hits[0].path, RelPath::new("Sources/Cast.swift"));
    assert_eq!(hits[0].line, Some(2));
}

#[test]
fn swiftlint_unknown_rule_is_not_available_naming_the_identifier() {
    assert_eq!(availability(Engine::SwiftLint), Availability::Available);
    let folder = std::env::temp_dir().join(format!("engines-zit-{}-swiftlint-unknown", std::process::id()));
    fs::create_dir(&folder).unwrap();
    let folder = fs::canonicalize(folder).unwrap();
    fs::write(folder.join("Example.swift"), "let value = 1\n").unwrap();
    fs::write(folder.join("temperare.toml"), "version = 1\n[[rule]]\nid = \"unknown-swift-rule\"\nengine = \"swiftlint\"\ncheck = \"no_such_rule\"\nfix = \"Name a known rule.\"\n").unwrap();
    let file = RuleFile::read(&folder.join("temperare.toml")).unwrap();
    // The version is machine-specific, so read the engine for this expected value only.
    let version_output = procio::run(&procio::Cmd {
        args: vec!["version".into()], cwd: Some(folder.clone()), ..procio::Cmd::new("swiftlint")
    }).unwrap();
    let outcomes = run_rules_counted(&file, &folder, &[]).into_iter().map(|run| (run.rule, run.outcome)).collect::<Vec<_>>();
    fs::remove_dir_all(&folder).unwrap();
    assert_eq!(outcomes.len(), 1, "{outcomes:?}");
    assert_eq!(outcomes[0].0, "unknown-swift-rule");
    let RuleOutcome::NotAvailable { rule, engine, why } = &outcomes[0].1 else {
        panic!("expected NotAvailable, got {:?}", outcomes[0].1);
    };
    assert_eq!((rule.as_str(), *engine), ("unknown-swift-rule", Engine::SwiftLint));
    assert_eq!(version_output.code, Some(0));
    let stdout = String::from_utf8(version_output.stdout).unwrap();
    let version = stdout.lines().next().unwrap().split_whitespace().next().unwrap();
    assert!(why.starts_with(&format!("swiftlint {version} does not know the check `no_such_rule`: ")), "{why}");
    assert!(
        why.ends_with(" does not know the check `no_such_rule`: warning: 'no_such_rule' is not a valid rule identifier"),
        "{why}"
    );
}

#[test]
fn d1_swiftlint_comma_separated_identifier_is_an_error() {
    assert_eq!(availability(Engine::SwiftLint), Availability::Available);
    let folder = std::env::temp_dir().join(format!("engines-zit-{}-swiftlint-invalid", std::process::id()));
    fs::create_dir(&folder).unwrap();
    let folder = fs::canonicalize(folder).unwrap();
    fs::write(folder.join("Example.swift"), "let value = 1\n").unwrap();
    let result = procio::run(&entry_cmd(&SWIFTLINT, OsStr::new("force_cast,force_try"), &[], None, &folder))
        .map(|out| parse_swiftlint(&out, "force_cast,force_try", &folder, &folder));
    fs::remove_dir_all(&folder).unwrap();
    let got = result.unwrap();
    assert!(got.is_err(), "{got:?}");
    assert!(got.unwrap_err().why.contains("is not a valid rule identifier"));
}
