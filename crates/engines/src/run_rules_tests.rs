//! ZUT for run_rules.rs (`run_rules_with`): outcomes in rule-file order over mixed engines, findings filled and cut,
//! absent and Tessera rules not available, garbage output failed, and an unresolvable folder. The host is a fake; no
//! engine runs. The rule file's folder is a temp folder the test makes and removes; ruff's output is hand-built.

use crate::host::Host;
use crate::run_rules::run_rules_with;
use procio::{Cmd, Output, ProcError};
use rules::{Engine, Finding, Kind, RelPath, RuleFile, RuleOutcome, Severity};
use std::cell::Cell;
use std::fs;
use std::path::{Path, PathBuf};

/// `py/` belongs to py-zone, the rest to repo. Rule order: ruff findings, tessera, an absent swiftlint, a ruff rule
/// whose run prints garbage.
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
    let stdout = if cmd.args.iter().any(|arg| arg == "E711") { RUFF_E711 } else { "garbage" };
    let code = if stdout == "garbage" { 2 } else { 1 };
    Ok(Output { code: Some(code), stdout: stdout.as_bytes().to_vec(), stderr: Vec::new() })
}

fn run(name: &str, only: &[&str]) -> Vec<(String, RuleOutcome)> {
    let repo = tree(name);
    let file = RuleFile::parse(TEXT, Path::new("lint")).unwrap();
    let only: Vec<RelPath> = only.iter().map(|path| RelPath::new(path)).collect();
    let on_path = |program: &str| program == "ruff";
    let got = run_rules_with(&file, &repo, &only, &Host { run: &fake_run, on_path: &on_path });
    fs::remove_dir_all(&repo).unwrap();
    got
}

fn finding(owner: &str, path: &str, line: u32, col: u32, what: &str) -> Finding {
    Finding {
        rule: "none-compare".to_string(),
        engine: Engine::Ruff,
        kind: Kind::Quality,
        severity: Severity::Warning,
        owner: Some(owner.to_string()),
        path: RelPath::new(path),
        line,
        col: Some(col),
        what: what.to_string(),
        fix: "Compare with `is None`.".to_string(),
    }
}

/// The three stopped outcomes, the same with or without `only`.
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
    let repo = tree("missing");
    let file = RuleFile::parse(TEXT, Path::new("gone")).unwrap();
    let runs = Cell::new(0);
    let count = |_: &Cmd| -> Result<Output, ProcError> {
        runs.set(runs.get() + 1);
        Err(ProcError::NotFound { program: "x".into() })
    };
    let on_path = |program: &str| program == "ruff";
    let got = run_rules_with(&file, &repo, &[], &Host { run: &count, on_path: &on_path });
    fs::remove_dir_all(&repo).unwrap();
    assert_eq!(runs.get(), 0);
    let ids: Vec<&str> = got.iter().map(|(id, _)| id.as_str()).collect();
    assert_eq!(ids, ["none-compare", "slop", "force-cast", "unused-import"]);
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
