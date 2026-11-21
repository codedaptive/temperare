//! ZIT for `run_rules_counted_with`, with each of its rule runs read as (rule, outcome):
//! outcomes in rule-file order over mixed engines, findings filled and cut, absent and Tessera rules not available,
//! garbage output failed, a rule marked `only_with_base` not available without a base, and an unresolvable folder.
//! Integration, not unit: each test makes and removes a real canonical temp folder for the rule file's repository. The
//! host is a fake and no engine runs; ruff's output is hand-built.

use crate::host::Host;
use crate::run_rules_counted_with::run_rules_counted_with;
use crate::types::NotAvailableCause;
use crate::RuleRun;
use procio::{Cmd, Output, ProcError};
use rules::{Engine, Finding, Kind, RelPath, RuleFile, RuleOutcome, Severity};
use std::cell::Cell;
use std::fs;
use std::path::{Path, PathBuf};

/// `py/` belongs to py-zone, the rest to repo. Rule order: ruff findings, tessera, an absent swiftlint, a ruff rule
/// whose run prints garbage, a ruff rule marked `only_with_base`, which no test gives a base. `unused-import`'s config,
/// which the fake host never reads, keeps it from sharing a run with the other ruff rules.
const TEXT: &str = r#"version = 1
owner = "repo"
[kits.py]
paths = ["py/**"]
owner = "py-zone"
[[rule]]
id = "none-compare"
engine = "ruff"
check = "E711"
paths = ["py/**", "tools/**"]
severity = "warning"
fix = "Compare with `is None`."
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
[[rule]]
id = "unused-import"
engine = "ruff"
check = "F401"
config = "/no/such/engines-zut-unused-import.toml"
fix = "f"
[[rule]]
id = "base-only"
engine = "ruff"
check = "E712"
only_with_base = true
fix = "f"
"#;

/// Hand-built `ruff check --output-format json` output: paths relative to where ruff ran. `docs/d.py` is outside the
/// rule's paths.
const RUFF_E711: &str = r#"[
 {"code":"E711","filename":"tools/t.py","location":{"row":4,"column":9},"message":"to None (t)"},
 {"code":"E711","filename":"docs/d.py","location":{"row":1,"column":1},"message":"to None (d)"},
 {"code":"E711","filename":"py/a.py","location":{"row":3,"column":13},"message":"to None (a)"}
]"#;

/// A fresh, canonical temp folder for one test.
fn tree(name: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!("engines-zut-{}-run-rules-{name}", std::process::id()));
    fs::create_dir_all(root.join("lint")).unwrap();
    fs::canonicalize(root).unwrap()
}

/// Ruff only is on PATH. A run selecting E711 prints `RUFF_E711`; any other run prints garbage and exits 2.
fn fake_run(cmd: &Cmd) -> Result<Output, ProcError> {
    let stdout = if cmd.args.iter().any(|arg| arg == "--select=E711") { RUFF_E711 } else { "garbage" };
    let code = if stdout == "garbage" { 2 } else { 1 };
    Ok(Output { code: Some(code), stdout: stdout.as_bytes().to_vec(), stderr: Vec::new() })
}

fn run(name: &str, only: &[&str]) -> Vec<(String, RuleOutcome)> {
    let repo = tree(name);
    let file = RuleFile::parse(TEXT, Path::new("lint")).unwrap();
    let only: Vec<RelPath> = only.iter().map(|path| RelPath::new(path)).collect();
    let on_path = |program: &str| program == "ruff";
    let host = Host { run: &fake_run, on_path: &on_path, raw_chars: &Cell::new(None), base: None };
    let got = outcomes(run_rules_counted_with(&file, &repo, &only, &host, &[]));
    fs::remove_dir_all(&repo).unwrap();
    got
}

/// Each rule run read as (rule, outcome): its rule's id and outcome.
fn outcomes(runs: Vec<RuleRun>) -> Vec<(String, RuleOutcome)> {
    runs.into_iter().map(|run| (run.rule, run.outcome)).collect()
}

fn finding(owner: &str, path: &str, line: u32, col: u32, what: &str) -> Finding {
    let mut finding = Finding::new(
        "none-compare",
        (Engine::Ruff, Kind::Quality, Severity::Warning),
        RelPath::new(path),
        line,
        Some(col),
        what,
        "Compare with `is None`.",
    );
    finding.owner = Some(owner.to_string());
    finding
}

/// The four stopped outcomes, the same with or without `only`.
fn stopped() -> Vec<(String, RuleOutcome)> {
    let why = |rule: &str, engine, why: &str| RuleOutcome::NotAvailable {
        rule: rule.to_string(),
        engine,
        why: why.to_string(),
    };
    vec![
        ("slop".to_string(), why("slop", Engine::Tessera, "Tessera's slop records are not defined yet")),
        ("force-cast".to_string(), why("force-cast", Engine::SwiftLint, "swiftlint not found on PATH")),
        (
            "unused-import".to_string(),
            RuleOutcome::Failed {
                rule: "unused-import".to_string(),
                engine: Engine::Ruff,
                code: Some(2),
                stderr: "ruff printed no JSON findings: garbage".to_string(),
            },
        ),
        ("base-only".to_string(), why("base-only", Engine::Ruff, "reports only what a change adds; run with --base")),
    ]
}

#[test]
fn every_rule_has_its_outcome_in_rule_file_order() {
    let mut want = vec![(
        "none-compare".to_string(),
        RuleOutcome::Findings(vec![
            finding("repo", "tools/t.py", 4, 9, "to None (t)"),
            finding("py-zone", "py/a.py", 3, 13, "to None (a)"),
        ]),
    )];
    want.extend(stopped());
    assert_eq!(run("order", &[]), want);
}

#[test]
fn only_cuts_the_findings_to_the_listed_paths() {
    let mut want = vec![(
        "none-compare".to_string(),
        RuleOutcome::Findings(vec![finding("py-zone", "py/a.py", 3, 13, "to None (a)")]),
    )];
    want.extend(stopped());
    assert_eq!(run("only", &["py/a.py"]), want);
}

#[test]
fn a_folder_that_cannot_be_resolved_fails_the_available_rules_and_runs_nothing() {
    // `base-only` still needs a base first: not available with that why, not failed with the folder's message.
    let repo = tree("missing");
    let file = RuleFile::parse(TEXT, Path::new("gone")).unwrap();
    let runs = Cell::new(0);
    let count = |_: &Cmd| -> Result<Output, ProcError> {
        runs.set(runs.get() + 1);
        Err(ProcError::NotFound { program: "x".into() })
    };
    let on_path = |program: &str| program == "ruff";
    let host = Host { run: &count, on_path: &on_path, raw_chars: &Cell::new(None), base: None };
    let ran = run_rules_counted_with(&file, &repo, &[], &host, &[]);
    fs::remove_dir_all(&repo).unwrap();
    assert_eq!(runs.get(), 0);
    let base_only = RuleOutcome::NotAvailable {
        rule: "base-only".to_string(),
        engine: Engine::Ruff,
        why: "reports only what a change adds; run with --base".to_string(),
    };
    assert_eq!(
        ran[4],
        RuleRun::new("base-only".to_string(), Engine::Ruff, base_only, None).with_cause(NotAvailableCause::Other)
    );
    let got = outcomes(ran);
    let ids: Vec<&str> = got.iter().map(|(id, _)| id.as_str()).collect();
    assert_eq!(ids, ["none-compare", "slop", "force-cast", "unused-import", "base-only"]);
    assert_eq!(got[1..3], stopped()[..2]);
    let start = format!("cannot resolve the rule file's folder {}: ", repo.join("gone").display());
    for (id, outcome) in [&got[0], &got[3]] {
        let RuleOutcome::Failed { rule, engine: Engine::Ruff, code: None, stderr } = outcome else {
            panic!("{id}: {outcome:?}")
        };
        assert_eq!(rule, id);
        assert!(stderr.starts_with(&start), "{id}: {stderr}");
    }
}
