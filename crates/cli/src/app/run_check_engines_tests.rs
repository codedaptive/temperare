//! Unit tests for run_check.rs: the engines the check seam names that the availability seam finds here, each with its version
//! probe, reach the printed report whatever their rules' outcomes, as text and as JSON, in the order given, and leave
//! the savings baseline alone.
//! The report seam returns a report with no engines, so every engine printed came from the check seam through
//! run_check.

use super::run_check_savings_tests::{assert_clean_body, check, printed_body, run, world};
use super::CheckWorld;
use crate::CheckArgs;
use engines::{Availability, CheckedRules, EngineVersion, RuleRun};
use procio::Oid;
use report::{NotAvailable, Report, ReportError};
use rules::{Engine, RelPath, RuleFile, RuleOutcome};
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
    let (before, rest) = out.split_once("engines: ruff 0.17.4, swiftlint (version unknown)\n").unwrap_or_else(|| panic!("{out}"));
    assert!(before.is_empty() || before.ends_with('\n'), "{out}");
    assert_clean_body(rest);
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

/// SwiftLint not found on PATH; every other engine found.
fn swiftlint_absent(engine: Engine) -> Availability {
    match engine {
        Engine::SwiftLint => Availability::NotAvailable { why: "swiftlint not found on PATH".to_string() },
        _ => Availability::Available,
    }
}

/// A report whose not_available carries SwiftLint's rule, as build_report gives it for an absent engine.
fn swiftlint_not_available(
    _: &RuleFile,
    _: Vec<(String, RuleOutcome)>,
    _: Option<(&Path, &Oid)>,
) -> Result<Report, ReportError> {
    let why = "swiftlint not found on PATH".to_string();
    let not_available = vec![NotAvailable { rule: "s".to_string(), engine: Engine::SwiftLint, why }];
    Ok(Report { not_available, ..Report::default() })
}

#[test]
fn json_leaves_an_absent_engine_out_of_engines_while_not_available_carries_it() {
    let seam = |_: &RuleFile, _: &Path, _: &[RelPath]| ruff_and_swiftlint();
    let w = CheckWorld {
        run_rules: &seam,
        availability: &swiftlint_absent,
        build_report: &swiftlint_not_available,
        ..world()
    };
    let (_, out, err) = check(&CheckArgs { json: true, ..CheckArgs::default() }, &w);
    assert_eq!(err, "");
    let printed: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(printed["engines"], json!([{"engine": "ruff", "version": "0.17.4"}]));
    assert_eq!(printed["not_available"][0]["engine"], "swiftlint");
}

#[test]
fn text_leaves_an_absent_engine_out_of_the_engines_line() {
    let seam = |_: &RuleFile, _: &Path, _: &[RelPath]| ruff_and_swiftlint();
    let w = CheckWorld {
        run_rules: &seam,
        availability: &swiftlint_absent,
        build_report: &swiftlint_not_available,
        ..world()
    };
    let (_, out, err) = check(&CheckArgs::default(), &w);
    assert_eq!(err, "");
    let engines_lines: Vec<&str> = out.lines().filter(|line| line.starts_with("engines:")).collect();
    assert_eq!(engines_lines, vec!["engines: ruff 0.17.4"], "{out}");
}

#[test]
fn json_prints_the_engines_in_first_use_order() {
    // The first-use order SwiftLint, AstGrep, Ruff, Clippy disagrees with the serialized-name sort (ast-grep, clippy,
    // ruff, swiftlint), with its reverse (swiftlint, ruff, clippy, ast-grep), and with the order the runs' rule ids
    // sort to (a ruff, b clippy, c swiftlint, d ast-grep).
    let seam = |_: &RuleFile, _: &Path, _: &[RelPath]| CheckedRules {
        runs: vec![
            run("c", Engine::SwiftLint, None),
            run("d", Engine::AstGrep, None),
            run("a", Engine::Ruff, Some(400)),
            run("b", Engine::Clippy, None),
        ],
        engines: vec![
            EngineVersion { engine: Engine::SwiftLint, version: Some("0.57.0".to_string()) },
            EngineVersion { engine: Engine::AstGrep, version: None },
            EngineVersion { engine: Engine::Ruff, version: Some("0.17.4".to_string()) },
            EngineVersion { engine: Engine::Clippy, version: Some("0.1.83".to_string()) },
        ],
    };
    let (code, out, err) =
        check(&CheckArgs { json: true, ..CheckArgs::default() }, &CheckWorld { run_rules: &seam, ..world() });
    assert_eq!((code, err.as_str()), (0, ""));
    assert!(
        out.contains(concat!(
            r#""engines":[{"engine":"swiftlint","version":"0.57.0"},{"engine":"ast-grep","version":null},"#,
            r#"{"engine":"ruff","version":"0.17.4"},{"engine":"clippy","version":"0.1.83"}]"#
        )),
        "{out}"
    );
}

const UNKNOWN_CHECK: &str = "ruff 0.17.4 does not know the check `ZZZ999`: ruff rule selection error";

/// Ruff found and probed at 0.17.4, its one rule not available because ruff does not know the rule's check.
fn ruff_without_the_check(_: &RuleFile, _: &Path, _: &[RelPath]) -> CheckedRules {
    let why = UNKNOWN_CHECK.to_string();
    let outcome = RuleOutcome::NotAvailable { rule: "q".to_string(), engine: Engine::Ruff, why };
    CheckedRules {
        runs: vec![RuleRun::new("q".to_string(), Engine::Ruff, outcome, Some(400))],
        engines: vec![EngineVersion { engine: Engine::Ruff, version: Some("0.17.4".to_string()) }],
    }
}

/// A report whose not_available carries ruff's rule, as build_report gives it for a check the engine does not know.
fn ruff_check_not_available(
    _: &RuleFile,
    _: Vec<(String, RuleOutcome)>,
    _: Option<(&Path, &Oid)>,
) -> Result<Report, ReportError> {
    let why = UNKNOWN_CHECK.to_string();
    let not_available = vec![NotAvailable { rule: "q".to_string(), engine: Engine::Ruff, why }];
    Ok(Report { not_available, ..Report::default() })
}

#[test]
fn text_keeps_a_found_engine_whose_rule_check_it_does_not_know() {
    let w = CheckWorld { run_rules: &ruff_without_the_check, build_report: &ruff_check_not_available, ..world() };
    let (code, out, err) = check(&CheckArgs::default(), &w);
    assert_eq!((code, err.as_str()), (3, ""), "{out}");
    let engines_lines: Vec<&str> = out.lines().filter(|line| line.starts_with("engines:")).collect();
    assert_eq!(engines_lines, vec!["engines: ruff 0.17.4"], "{out}");
}

#[test]
fn json_keeps_a_found_engine_whose_rule_check_it_does_not_know() {
    let w = CheckWorld { run_rules: &ruff_without_the_check, build_report: &ruff_check_not_available, ..world() };
    let (code, out, err) = check(&CheckArgs { json: true, ..CheckArgs::default() }, &w);
    assert_eq!((code, err.as_str()), (3, ""), "{out}");
    let printed: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(printed["engines"], json!([{"engine": "ruff", "version": "0.17.4"}]));
    assert_eq!(printed["not_available"][0]["engine"], "ruff");
}
