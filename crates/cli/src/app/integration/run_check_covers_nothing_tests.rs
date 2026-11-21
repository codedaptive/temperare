//! No-file engine runs reach the printed check report.

use super::super::run_check_savings_tests::{assert_counts_tail, check, checked, world};
use super::super::CheckWorld;
use crate::CheckArgs;
use engines::RuleRun;
use rules::{Engine, RelPath, Rule, RuleFile, RuleOutcome};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

#[test]
fn run_check_reports_rules_with_no_files() {
    let rows = [
        (
            "own-scripts",
            6,
            "covers nothing: own-scripts (shellcheck)",
            json!([{"rule": "own-scripts", "engine": "shellcheck"}]),
            json!([]),
            (0, 1),
            "fix the paths of each rule in covers_nothing, or remove it from /repo/temperare.toml",
        ),
        (
            "tm-house-shell",
            0,
            "not applicable: tm-house-shell (shellcheck): no file its engine checks",
            json!([]),
            json!([{"rule": "tm-house-shell", "engine": "shellcheck", "why": "no file its engine checks"}]),
            (1, 0),
            "nothing to do",
        ),
    ];
    for (id, exit, line, covers_nothing, not_applicable, (na_count, cn_count), next) in rows {
        let mut file = RuleFile::new(PathBuf::from("/repo"));
        file.rules.push(Rule::new(id, Engine::ShellCheck, "shellcheck"));
        let read = |_: &Path| Ok(file.clone());
        let runs = |_: &RuleFile, _: &Path, _: &[RelPath], _: Option<&str>| {
            let mut run = RuleRun::new(id.into(), Engine::ShellCheck, RuleOutcome::Findings(vec![]), None);
            run.no_files = Some("no file its engine checks".into());
            checked(vec![run])
        };
        let w = CheckWorld { read: &read, run_rules: &runs, build_report: &report::build_report_with_no_files, ..world() };

        let (code, out, err) = check(&CheckArgs::default(), &w);
        assert_eq!((code, err.as_str()), (exit, ""), "{id}: {out}");
        assert!(out.lines().any(|printed| printed == line), "{id}: {out}");
        assert_counts_tail(&out, na_count, cn_count);
        assert!(out.lines().any(|printed| printed.starts_with("0 findings (")), "{id}: {out}");

        let (code, out, err) = check(&CheckArgs { json: true, ..CheckArgs::default() }, &w);
        assert_eq!((code, err.as_str()), (exit, ""), "{id}: {out}");
        let value: Value = serde_json::from_str(&out).unwrap();
        assert_eq!(value["next"], next, "{id}");
        assert_eq!(value["covers_nothing"], covers_nothing, "{id}");
        assert_eq!(value["not_applicable"], not_applicable, "{id}");
        assert_eq!(value["not_available"], json!([]), "{id}");
        assert_eq!(value["findings"], json!([]), "{id}");
    }
}
