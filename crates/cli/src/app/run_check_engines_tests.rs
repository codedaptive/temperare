//! ZUT for run_check.rs: the engines the check seam names reach the printed report, as text and as JSON, and leave
//! the savings baseline alone. The report seam returns a report with no engines, so every engine printed came from
//! the check seam through run_check.

use super::run_check_savings_tests::{check, printed_body, run, world, BODY};
use super::CheckWorld;
use crate::CheckArgs;
use engines::{CheckedRules, EngineVersion};
use rules::{Engine, RelPath, RuleFile};
use serde_json::json;
use std::path::Path;

/// One ruff run that printed 400 characters, and the given engines.
fn ruff_run_naming(engines: Vec<EngineVersion>) -> CheckedRules {
    CheckedRules { runs: vec![run("q", Engine::Ruff, Some(400))], engines }
}

/// Ruff with a version read and SwiftLint with none.
fn ruff_and_swiftlint() -> CheckedRules {
    ruff_run_naming(vec![
        EngineVersion { engine: Engine::Ruff, version: Some("0.17.4".to_string()) },
        EngineVersion { engine: Engine::SwiftLint, version: None },
    ])
}

#[test]
fn json_prints_each_engine_with_its_version_or_null() {
    let seam = |_: &RuleFile, _: &Path, _: &[RelPath]| ruff_and_swiftlint();
    let (code, out, err) = check(&CheckArgs { json: true, ..CheckArgs::default() }, &CheckWorld { run_rules: &seam, ..world() });
    assert_eq!((code, err.as_str()), (0, ""));
    assert!(
        out.contains(r#""engines":[{"engine":"ruff","version":"0.17.4"},{"engine":"swiftlint","version":null}]"#),
        "{out}"
    );
}

#[test]
fn text_prints_the_engines_line_just_before_the_counts_line() {
    let seam = |_: &RuleFile, _: &Path, _: &[RelPath]| ruff_and_swiftlint();
    let (code, out, err) = check(&CheckArgs::default(), &CheckWorld { run_rules: &seam, ..world() });
    assert_eq!((code, err.as_str()), (0, ""));
    assert!(out.ends_with(&format!("engines: ruff 0.17.4, swiftlint (version unknown)\n{BODY}")), "{out}");
}

#[test]
fn engines_leave_the_baseline_runs_and_counted_alone_and_output_counts_the_printed_body() {
    let bare = |_: &RuleFile, _: &Path, _: &[RelPath]| ruff_run_naming(vec![]);
    let named = |_: &RuleFile, _: &Path, _: &[RelPath]| ruff_and_swiftlint();
    let args = CheckArgs { json: true, ..CheckArgs::default() };
    for seam in [&bare as &dyn Fn(&RuleFile, &Path, &[RelPath]) -> CheckedRules, &named] {
        let (code, out, err) = check(&args, &CheckWorld { run_rules: seam, ..world() });
        assert_eq!((code, err.as_str()), (0, ""));
        let (body, saved) = printed_body(&out);
        assert_eq!(saved["baseline_chars"], 400);
        assert_eq!(saved["baseline_tokens"], 100);
        assert_eq!(saved["engine_runs"], 1);
        assert_eq!(saved["counted"], json!([{"rule": "q", "engine": "ruff", "chars": 400}]));
        assert_eq!(saved["output_chars"], body.chars().count());
    }
}
