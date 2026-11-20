//! ZIT for run_rules_counted_with's grouping: rules that share a run make one host run and one count, through the real folder resolution (the rule file's folder is "/", which canonicalizes).

use crate::host::Host;
use crate::run_rules_counted_with::run_rules_counted_with;
use crate::types::RuleRun;
use procio::{Cmd, Output, ProcError};
use rules::{Engine, Finding, Kind, RelPath, RuleFile, RuleOutcome, Severity};
use std::cell::{Cell, RefCell};
use std::path::Path;

/// Two ruff rules that share, and between them one whose config (never read by the fake) keeps it apart.
const SHARING: &str = r#"version = 1
[[rule]]
id = "none-compare"
engine = "ruff"
check = "E711"
fix = "f"
[[rule]]
id = "long-line"
engine = "ruff"
check = "E501"
config = "/no/such/engines-zut-long-line.toml"
fix = "g"
[[rule]]
id = "unused"
engine = "ruff"
check = "F401"
fix = "h"
"#;

/// The shared run's output, 161 characters, with a hit for each sharing rule.
const SHARED_OUT: &str = r#"[{"code":"E711","filename":"a.py","location":{"row":1,"column":2},"message":"n"},{"code":"F401","filename":"b.py","location":{"row":3,"column":1},"message":"u"}]"#;

/// The apart rule's own output, 81 characters.
const OWN_OUT: &str = r#"[{"code":"E501","filename":"c.py","location":{"row":2,"column":1},"message":"l"}]"#;

#[test]
fn sharing_rules_make_one_counted_run_and_a_rule_between_them_keeps_its_own() {
    let file = RuleFile::parse(SHARING, Path::new(".")).unwrap();
    let selects = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        let select = cmd.args.iter().find(|arg| arg.to_string_lossy().starts_with("--select=")).unwrap();
        let select = select.to_string_lossy().into_owned();
        let stdout = match select.as_str() {
            "--select=E711,F401" => SHARED_OUT,
            "--select=E501" => OWN_OUT,
            other => panic!("unexpected check run: {other}"),
        };
        selects.borrow_mut().push(select);
        Ok(Output { code: Some(1), stdout: stdout.as_bytes().to_vec(), stderr: vec![] })
    };
    let host = Host { run: &run, on_path: &|program| program == "ruff", raw_chars: &Cell::new(None) };
    let got = run_rules_counted_with(&file, Path::new("/"), &[], &host, &[]);
    assert_eq!(selects.into_inner(), ["--select=E711,F401", "--select=E501"]);
    let finding = |id: &str, path: &str, line: u32, col: u32, what: &str, fix: &str| {
        Finding::new(id, (Engine::Ruff, Kind::Quality, Severity::Error), RelPath::new(path), line, Some(col), what, fix)
    };
    assert_eq!(got, vec![
        RuleRun {
            rule: "none-compare".into(),
            engine: Engine::Ruff,
            outcome: RuleOutcome::Findings(vec![finding("none-compare", "a.py", 1, 2, "n", "f")]),
            raw_chars: Some(161),
        },
        RuleRun {
            rule: "long-line".into(),
            engine: Engine::Ruff,
            outcome: RuleOutcome::Findings(vec![finding("long-line", "c.py", 2, 1, "l", "g")]),
            raw_chars: Some(81),
        },
        RuleRun {
            rule: "unused".into(),
            engine: Engine::Ruff,
            outcome: RuleOutcome::Findings(vec![finding("unused", "b.py", 3, 1, "u", "h")]),
            raw_chars: None,
        },
    ]);
}
