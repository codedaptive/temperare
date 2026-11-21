//! Kind selection carries through to the printed not-applicable report.

use super::super::run_check_savings_tests::{check, checked, run, world};
use super::super::CheckWorld;
use crate::CheckArgs;
use rules::{Engine, Kind, RelPath, Rule, RuleFile};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

#[test]
fn run_check_kind_not_applicable_filters_printed_list_and_count() {
    let mut file = RuleFile::new(PathBuf::from("/repo"));
    let mut runnable = Rule::new("security-run", Engine::AstGrep, "no-eval");
    runnable.kind = Kind::Security;
    file.rules.push(runnable);
    let mut security = Rule::new("security-na", Engine::AstGrep, "no-eval");
    security.kind = Kind::Security;
    security.not_applicable = Some("security parameter missing".into());
    let mut quality = Rule::new("quality-na", Engine::Ruff, "E501");
    quality.not_applicable = Some("quality parameter missing".into());
    file.not_applicable_rules = vec![security, quality];

    let read = |_: &Path| Ok(file.clone());
    let runs = |_: &RuleFile, _: &Path, _: &[RelPath]| {
        checked(vec![run("security-run", Engine::AstGrep, None)])
    };
    let w = CheckWorld { read: &read, run_rules: &runs, build_report: &report::build_report, ..world() };

    let (code, out, err) = check(&CheckArgs { json: true, ..CheckArgs::default() }, &w);
    assert_eq!((code, err.as_str()), (0, ""));
    let all: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(all["not_applicable"], json!([
        {"rule": "security-na", "engine": "ast-grep", "why": "security parameter missing"},
        {"rule": "quality-na", "engine": "ruff", "why": "quality parameter missing"}
    ]));
    assert_eq!(all["counts"]["not_applicable"], 2);

    let (code, out, err) = check(&CheckArgs { kind: Some(Kind::Security), json: true, ..CheckArgs::default() }, &w);
    assert_eq!((code, err.as_str()), (0, ""));
    let selected: Value = serde_json::from_str(&out).unwrap();
    assert_eq!(selected["not_applicable"], json!([
        {"rule": "security-na", "engine": "ast-grep", "why": "security parameter missing"}
    ]));
    assert_eq!(selected["counts"]["not_applicable"], 1);
}
