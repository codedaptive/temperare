//! ZUT for rule_run_of.rs: each kind of flow result made by hand, read into one rule's run. Nothing runs.

use crate::rule_run_of::rule_run_of;
use crate::types::{EngineVersion, Hit, RuleRun, Stop};
use rules::{Engine, Finding, Kind, RelPath, Rule, RuleFile, RuleOutcome, Severity};
use std::path::Path;

const TEXT: &str = r#"version = 1
owner = "repo"
[[rule]]
id = "none-compare"
engine = "ruff"
check = "E711"
paths = ["src/**"]
fix = "f"
"#;

fn file() -> (RuleFile, Rule) {
    let file = RuleFile::parse(TEXT, Path::new(".")).unwrap();
    let rule = file.rules[0].clone();
    (file, rule)
}

fn hit(path: &str) -> Hit {
    Hit { path: RelPath::new(path), line: Some(2), col: Some(4), what: "none".into() }
}

#[test]
fn hits_are_its_findings_inside_its_paths_with_no_flag() {
    let (file, rule) = file();
    let got = rule_run_of(&file, &rule, Ok(vec![hit("src/a.py"), hit("lib/b.py")]), &[], &[], Some(88));
    let mut finding =
        Finding::new("none-compare", (Engine::Ruff, Kind::Quality, Severity::Error), RelPath::new("src/a.py"), 2, Some(4), "none", "f");
    finding.owner = Some("repo".into());
    let want = RuleRun::new("none-compare".into(), Engine::Ruff, RuleOutcome::Findings(vec![finding]), Some(88));
    assert_eq!(got, want);
}

#[test]
fn no_files_is_no_findings_with_its_why_as_the_flag() {
    let (file, rule) = file();
    let got = rule_run_of(&file, &rule, Err(Stop::NoFiles { why: "no file its engine checks".into() }), &[], &[], None);
    assert_eq!(got.rule, "none-compare");
    assert_eq!(got.engine, Engine::Ruff);
    assert_eq!(got.outcome, RuleOutcome::Findings(vec![]));
    assert_eq!(got.raw_chars, None);
    assert_eq!(got.no_files, Some("no file its engine checks".to_string()));
}

#[test]
fn another_stop_is_its_outcome_with_no_flag() {
    let (file, rule) = file();
    let versions = [EngineVersion { engine: Engine::Ruff, version: Some("0.17.4".into()) }];
    let got = rule_run_of(&file, &rule, Err(Stop::UnknownCheck { said: "unknown".into() }), &[], &versions, Some(77));
    let why = "ruff 0.17.4 does not know the check `E711`: unknown";
    let outcome = RuleOutcome::NotAvailable { rule: "none-compare".into(), engine: Engine::Ruff, why: why.into() };
    assert_eq!(got, RuleRun::new("none-compare".into(), Engine::Ruff, outcome, Some(77)));
}

#[test]
fn a_failed_stop_keeps_its_code_and_stderr() {
    let (file, rule) = file();
    let got = rule_run_of(&file, &rule, Err(Stop::Failed { code: Some(2), stderr: "boom".into() }), &[], &[], None);
    let outcome = RuleOutcome::Failed { rule: "none-compare".into(), engine: Engine::Ruff, code: Some(2), stderr: "boom".into() };
    assert_eq!(got, RuleRun::new("none-compare".into(), Engine::Ruff, outcome, None));
}
