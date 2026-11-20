//! ZIT for run_rules_versioned_with.rs: the engines' versions probed once each before the rules run, the probes not
//! counted in any run, and a too-old rule's why naming the version read. The host is fake, so no engine runs; the rule
//! file's folder is `/`, which always resolves.

use crate::host::Host;
use crate::run_rules_versioned_with::run_rules_versioned_with;
use crate::types::{CheckedRules, EngineVersion, RuleRun};
use procio::{Cmd, Output, ProcError};
use rules::{Engine, RuleFile, RuleOutcome};
use std::cell::{Cell, RefCell};
use std::path::Path;

const RULES: &str = r#"version = 1
[[rule]]
id = "clean"
engine = "ruff"
check = "E711"
fix = "f"
[[rule]]
id = "too-old"
engine = "ruff"
check = "ZZZ999"
fix = "f"
[[rule]]
id = "slop"
engine = "tessera"
check = "slop"
fix = "f"
"#;

fn output(code: i32, stdout: &str, stderr: &str) -> Result<Output, ProcError> {
    Ok(Output { code: Some(code), stdout: stdout.as_bytes().to_vec(), stderr: stderr.as_bytes().to_vec() })
}

#[test]
fn versions_are_probed_once_uncounted_and_name_the_too_old_check() {
    let file = RuleFile::parse(RULES, Path::new(".")).unwrap();
    // 1000 characters: "ruff 1.2.3 " (11) then 989 more.
    let probe_stdout = format!("ruff 1.2.3 {}", "x".repeat(989));
    let probes = RefCell::new(0);
    let run = |cmd: &Cmd| {
        if cmd.args == vec!["--version"] {
            *probes.borrow_mut() += 1;
            output(0, &probe_stdout, "")
        } else if cmd.args.iter().any(|arg| arg == "--select=E711") {
            output(0, "[]", "")
        } else if cmd.args.iter().any(|arg| arg == "--select=ZZZ999") {
            output(2, "", "  Cause: Unknown rule selector `ZZZ999` in `select` from the CLI")
        } else {
            panic!("unexpected command: {cmd:?}")
        }
    };
    let host = Host { run: &run, on_path: &|program| program == "ruff", raw_chars: &Cell::new(None) };
    let got = run_rules_versioned_with(&file, Path::new("/"), &[], &host);
    assert_eq!(got, CheckedRules {
        runs: vec![
            RuleRun {
                rule: "clean".into(),
                engine: Engine::Ruff,
                outcome: RuleOutcome::Findings(vec![]),
                raw_chars: Some(2),
            },
            RuleRun {
                rule: "too-old".into(),
                engine: Engine::Ruff,
                outcome: RuleOutcome::NotAvailable {
                    rule: "too-old".into(),
                    engine: Engine::Ruff,
                    why: "ruff 1.2.3 does not know the check `ZZZ999`: Cause: Unknown rule selector `ZZZ999` in `select` from the CLI".into(),
                },
                raw_chars: Some(64),
            },
            RuleRun {
                rule: "slop".into(),
                engine: Engine::Tessera,
                outcome: RuleOutcome::NotAvailable {
                    rule: "slop".into(),
                    engine: Engine::Tessera,
                    why: crate::entry_for(Engine::Tessera).unwrap().never.unwrap().into(),
                },
                raw_chars: None,
            },
        ],
        engines: vec![
            EngineVersion { engine: Engine::Ruff, version: Some("1.2.3".into()) },
            EngineVersion { engine: Engine::Tessera, version: None },
        ],
    });
    assert_eq!(probes.into_inner(), 1);
}

#[test]
fn an_unresolved_folder_probes_nothing_and_fails_the_runs() {
    let file = RuleFile::parse(RULES, Path::new("no/such/engines-zut-folder")).unwrap();
    let run = |cmd: &Cmd| -> Result<Output, ProcError> { panic!("nothing may run: {cmd:?}") };
    let host = Host { run: &run, on_path: &|_| true, raw_chars: &Cell::new(None) };
    let got = run_rules_versioned_with(&file, Path::new("/"), &[], &host);
    assert_eq!(got.engines, vec![
        EngineVersion { engine: Engine::Ruff, version: None },
        EngineVersion { engine: Engine::Tessera, version: None },
    ]);
    assert!(matches!(got.runs[0].outcome, RuleOutcome::Failed { code: None, .. }), "{:?}", got.runs[0]);
}
