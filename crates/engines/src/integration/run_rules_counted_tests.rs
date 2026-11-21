//! Counted rule runs across adapters with a fake host and a real temporary rule folder.

use crate::host::Host;
use crate::run_rules_counted_with::run_rules_counted_with;
use crate::RuleRun;
use procio::{Cmd, Output, ProcError};
use rules::{Engine, Finding, Kind, RelPath, RuleFile, RuleOutcome, Severity};
use std::cell::Cell;
use std::fs;
use std::path::Path;

/// `bad-output` and `empty-output` each name a config of their own, which the fake host never reads: it keeps the
/// ruff rules from sharing a run, so each prints its own output. `refused` shares with `found` but stops before
/// running, so `found`'s run selects E711 alone.
const RULES: &str = r#"version = 1
owner = "repo"
[[rule]]
id = "found"
engine = "ruff"
check = "E711"
severity = "warning"
fix = "f"
[[rule]]
id = "bad-output"
engine = "ruff"
check = "F401"
config = "/no/such/engines-zit-bad-output.toml"
fix = "f"
[[rule]]
id = "empty-output"
engine = "ruff"
check = "E501"
config = "/no/such/engines-zit-empty-output.toml"
fix = "f"
[[rule]]
id = "refused"
engine = "ruff"
check = "flake8-bugbear"
fix = "f"
[[rule]]
id = "slop"
engine = "tessera"
check = "slop"
fix = "f"
[[rule]]
id = "absent"
engine = "swiftlint"
check = "force_cast"
fix = "f"
"#;

const FINDINGS: &str = r#"[{"code":"E711","filename":"a.py","location":{"row":1,"column":2},"message":"é"}]"#;

#[test]
fn counted_runs_keep_order_outcomes_and_distinguish_empty_from_not_run() {
    let root = std::env::temp_dir().join(format!("engines-zit-{}-counted-rules", std::process::id()));
    fs::create_dir(&root).unwrap();
    let repo = fs::canonicalize(root).unwrap();
    let file = RuleFile::parse(RULES, Path::new(".")).unwrap();
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        if cmd.args.iter().any(|arg| arg == "--select=E711") {
            Ok(Output { code: Some(1), stdout: FINDINGS.as_bytes().to_vec(), stderr: vec![] })
        } else if cmd.args.iter().any(|arg| arg == "--select=F401") {
            Ok(Output { code: Some(2), stdout: b"bad\xff".to_vec(), stderr: b"oops".to_vec() })
        } else if cmd.args.iter().any(|arg| arg == "--select=E501") {
            Ok(Output { code: Some(0), stdout: vec![], stderr: vec![] })
        } else {
            panic!("unexpected check command: {cmd:?}")
        }
    };
    let raw_chars = Cell::new(None);
    let host = Host { run: &run, on_path: &|program| program == "ruff", raw_chars: &raw_chars, base: None };
    let got = run_rules_counted_with(&file, &repo, &[], &host, &[]);
    fs::remove_dir_all(&repo).unwrap();

    let quality_warning = (Engine::Ruff, Kind::Quality, Severity::Warning);
    let mut found = Finding::new("found", quality_warning, RelPath::new("a.py"), 1, Some(2), "é", "f");
    found.owner = Some("repo".into());
    assert_eq!(got, vec![
        RuleRun::new(
            "found".into(),
            Engine::Ruff,
            RuleOutcome::Findings(vec![found]),
            Some(81),
        ),
        RuleRun::new(
            "bad-output".into(),
            Engine::Ruff,
            RuleOutcome::Failed { rule: "bad-output".into(), engine: Engine::Ruff, code: Some(2), stderr: "oops".into() },
            Some(8),
        ),
        RuleRun::new(
            "empty-output".into(),
            Engine::Ruff,
            RuleOutcome::Failed { rule: "empty-output".into(), engine: Engine::Ruff, code: Some(0), stderr: "ruff printed no JSON findings: ".into() },
            Some(0),
        ),
        RuleRun::new(
            "refused".into(),
            Engine::Ruff,
            RuleOutcome::Failed { rule: "refused".into(), engine: Engine::Ruff, code: None, stderr: "ruff check `flake8-bugbear`: `flake8-bugbear` is not a rule code, a code prefix or ALL, so tm cannot tell which findings it selects".into() },
            None,
        ),
        RuleRun::new(
            "slop".into(),
            Engine::Tessera,
            RuleOutcome::NotAvailable { rule: "slop".into(), engine: Engine::Tessera, why: "Tessera's slop records are not defined yet".into() },
            None,
        ),
        RuleRun::new(
            "absent".into(),
            Engine::SwiftLint,
            RuleOutcome::NotAvailable { rule: "absent".into(), engine: Engine::SwiftLint, why: "swiftlint not found on PATH".into() },
            None,
        ),
    ]);
    assert_eq!(got.iter().filter_map(|run| run.raw_chars).sum::<usize>(), 89);
    assert_eq!(got.iter().filter(|run| run.raw_chars.is_some()).count(), 3);
}
