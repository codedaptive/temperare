//! Zone integration tests for versioned rule runs: real engines check files made in a temporary repository.

use procio::{Cmd, Output};
use rules::{Engine, RuleFile, RuleOutcome};
use std::fs;
use std::path::Path;

const RULES: &str = r#"version = 1
owner = "repo"
[[rule]]
id = "none-compare"
engine = "ruff"
check = "E711"
fix = "f"
[[rule]]
id = "unknown-selector"
engine = "ruff"
check = "ZZZ999"
fix = "f"
[[rule]]
id = "slop"
engine = "tessera"
check = "slop"
fix = "f"
[[rule]]
id = "force-cast"
engine = "swiftlint"
check = "force_cast"
fix = "f"
"#;

#[test]
fn run_rules_versioned_at_reports_engines() {
    let root = std::env::temp_dir().join(format!(
        "engines-zit-{}-run_rules_versioned_at_reports_engines",
        std::process::id()
    ));
    fs::create_dir(&root).unwrap();
    let repo = fs::canonicalize(root).unwrap();
    fs::write(repo.join("a.py"), "x = 1\nx == None\n").unwrap();
    let file = RuleFile::parse(RULES, Path::new(".")).unwrap();

    // These expected versions are read from the engines only because they are machine-specific.
    let probe = |program: &str, arg: &str| -> Option<Output> {
        procio::which(program).map(|_| {
            procio::run(&Cmd { args: vec![arg.into()], cwd: Some(repo.clone()), ..Cmd::new(program) }).unwrap()
        })
    };
    let ruff_output = probe("ruff", "--version");
    let swiftlint_output = probe("swiftlint", "version");
    let got: crate::CheckedRules = crate::run_rules_versioned_at(&file, &repo, &[], None);
    fs::remove_dir_all(&repo).unwrap();

    let version = |output: Option<Output>, word: usize| -> Option<String> {
        output.map(|output| {
            assert_eq!(output.code, Some(0));
            String::from_utf8(output.stdout).unwrap().split_whitespace().nth(word).unwrap().to_string()
        })
    };
    let ruff_version = version(ruff_output, 1);
    let swiftlint_version = version(swiftlint_output, 0);
    assert_eq!(got.engines, vec![
        crate::EngineVersion { engine: Engine::Ruff, version: ruff_version.clone() },
        crate::EngineVersion { engine: Engine::Tessera, version: None },
        crate::EngineVersion { engine: Engine::SwiftLint, version: swiftlint_version },
    ]);
    assert_eq!(got.runs.len(), 4);
    match ruff_version {
        Some(version) => {
            let RuleOutcome::Findings(findings) = &got.runs[0].outcome else {
                panic!("expected ruff findings, got {:?}", got.runs[0].outcome);
            };
            assert_eq!(findings.len(), 1);
            let RuleOutcome::NotAvailable { why, .. } = &got.runs[1].outcome else {
                panic!("expected unknown selector, got {:?}", got.runs[1].outcome);
            };
            assert!(why.starts_with(&format!("ruff {version} does not know the check `ZZZ999`: ")), "{why}");
            assert!(got.runs[0].raw_chars.is_some());
            assert!(got.runs[1].raw_chars.is_some());
        }
        None => {
            for run in &got.runs[..2] {
                let RuleOutcome::NotAvailable { why, .. } = &run.outcome else {
                    panic!("expected missing ruff, got {:?}", run.outcome);
                };
                assert_eq!(why, "ruff not found on PATH");
            }
        }
    }
}
