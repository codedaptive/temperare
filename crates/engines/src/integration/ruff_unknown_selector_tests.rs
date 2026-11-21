//! Integration tests for an unknown ruff selector: real ruff runs on files the test made.

use procio::Cmd;
use rules::{Engine, RuleFile, RuleOutcome};
use std::fs;
use std::path::Path;

#[test]
fn ruff_unknown_selector_is_not_available() {
    let root = std::env::temp_dir().join(format!(
        "engines-zit-{}-ruff_unknown_selector_is_not_available",
        std::process::id()
    ));
    fs::create_dir(&root).unwrap();
    let repo = fs::canonicalize(root).unwrap();
    let text = r#"version = 1
owner = "repo"
[[rule]]
id = "unknown-selector"
engine = "ruff"
check = "ZZZ999"
fix = "f"
"#;
    fs::write(repo.join("temperare.toml"), text).unwrap();
    fs::write(repo.join("a.py"), "value = 1\n").unwrap();
    let file = RuleFile::parse(text, Path::new(".")).unwrap();
    let ruff_present = procio::which("ruff").is_some();
    // The version is machine-specific, so read the engine for this expected value only.
    let version_output = ruff_present.then(|| {
        procio::run(&Cmd { args: vec!["--version".into()], cwd: Some(repo.clone()), ..Cmd::new("ruff") })
            .unwrap()
    });
    let got = crate::run_rules_versioned_at(&file, &repo, &[], None).runs.into_iter().map(|run| (run.rule, run.outcome)).collect::<Vec<_>>();
    fs::remove_dir_all(&repo).unwrap();

    assert_eq!(got.len(), 1, "{got:?}");
    assert_eq!(got[0].0, "unknown-selector");
    let RuleOutcome::NotAvailable { rule, engine, why } = &got[0].1 else {
        panic!("expected NotAvailable, got {:?}", got[0].1);
    };
    assert_eq!(rule, "unknown-selector");
    assert_eq!(*engine, Engine::Ruff);
    match version_output {
        None => assert_eq!(why, "ruff not found on PATH"),
        Some(output) => {
            assert_eq!(output.code, Some(0));
            let stdout = String::from_utf8(output.stdout).unwrap();
            let version = stdout.split_whitespace().nth(1).unwrap();
            assert!(why.starts_with(&format!("ruff {version} does not know the check")), "{why}");
            assert!(why.contains("does not know the check `ZZZ999`: "), "{why}");
            assert!(why.contains("Unknown rule selector"), "{why}");
        }
    }
}
