//! ZIT for run_rules_versioned_with.rs: the engines' versions probed once each before the rules run, the probes not
//! counted in any run, and a too-old rule's why naming the version read. The two ruff rules of `RULES` share a run
//! that ruff fails on the unknown selector, so `too-old` counts that run and `clean` the rerun without it. The host is fake, so
//! no engine runs; the rule file's folder is `/`, which always resolves. A ShellCheck code absent from the list
//! also uses the single version probe, in a throwaway git repository the test makes and removes, holding one
//! script, since its scripts are listed before the code is looked up in the list. A rule marked `only_with_base` with
//! no base still has its engine probed, and no other host command or check runs.

use crate::host::Host;
use crate::run_rules_versioned_with::run_rules_versioned_with;
use crate::types::{CheckedRules, EngineVersion, NotAvailableCause, RuleRun};
use procio::{Cmd, Output, ProcError};
use rules::{Engine, RuleFile, RuleOutcome};
use std::cell::{Cell, RefCell};
use std::fs;
use std::path::Path;

#[path = "../../../tests/support/git.rs"]
mod git_support;

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
    let selects = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| {
        let select = cmd.args.iter().find(|arg| arg.to_string_lossy().starts_with("--select="));
        selects.borrow_mut().extend(select.map(|arg| arg.to_string_lossy().into_owned()));
        if cmd.args == vec!["--version"] {
            *probes.borrow_mut() += 1;
            output(0, &probe_stdout, "")
        } else if cmd.args.iter().any(|arg| arg == "--select=E711,ZZZ999") {
            // The two ruff rules share one run; ruff fails it on the selector it does not know.
            output(2, "", "  Cause: Unknown rule selector `ZZZ999` in `select` from the CLI")
        } else if cmd.args.iter().any(|arg| arg == "--select=E711") {
            // The rerun without the unknown selector, which gives `clean` its own counted run.
            output(0, "[]", "")
        } else {
            panic!("unexpected command: {cmd:?}")
        }
    };
    let host = Host { run: &run, on_path: &|program| program == "ruff", raw_chars: &Cell::new(None), base: None };
    let got = run_rules_versioned_with(&file, Path::new("/"), &[], &host);
    assert_eq!(got, CheckedRules {
        runs: vec![
            RuleRun::new(
                "clean".into(),
                Engine::Ruff,
                RuleOutcome::Findings(vec![]),
                Some(2),
            ),
            RuleRun::new(
                "too-old".into(),
                Engine::Ruff,
                RuleOutcome::NotAvailable {
                    rule: "too-old".into(),
                    engine: Engine::Ruff,
                    why: "ruff 1.2.3 does not know the check `ZZZ999`: Cause: Unknown rule selector `ZZZ999` in `select` from the CLI".into(),
                },
                Some(64),
            )
            .with_cause(NotAvailableCause::UnknownCheck),
            RuleRun::new(
                "slop".into(),
                Engine::Tessera,
                RuleOutcome::NotAvailable {
                    rule: "slop".into(),
                    engine: Engine::Tessera,
                    why: crate::entry_of(Engine::Tessera).never.unwrap().into(),
                },
                None,
            )
            .with_cause(NotAvailableCause::Other),
        ],
        engines: vec![
            EngineVersion { engine: Engine::Ruff, version: Some("1.2.3".into()) },
            EngineVersion { engine: Engine::Tessera, version: None },
        ],
    });
    assert_eq!(probes.into_inner(), 1);
    assert_eq!(selects.into_inner(), ["--select=E711,ZZZ999", "--select=E711"]);
}

#[test]
fn an_unresolved_folder_probes_nothing_and_fails_the_runs() {
    let file = RuleFile::parse(RULES, Path::new("no/such/engines-zut-folder")).unwrap();
    let run = |cmd: &Cmd| -> Result<Output, ProcError> { panic!("nothing may run: {cmd:?}") };
    let host = Host { run: &run, on_path: &|_| true, raw_chars: &Cell::new(None), base: None };
    let got = run_rules_versioned_with(&file, Path::new("/"), &[], &host);
    assert_eq!(got.engines, vec![
        EngineVersion { engine: Engine::Ruff, version: None },
        EngineVersion { engine: Engine::Tessera, version: None },
    ]);
    assert!(matches!(got.runs[0].outcome, RuleOutcome::Failed { code: None, .. }), "{:?}", got.runs[0]);
}

#[test]
fn shellcheck_unlisted_code_uses_one_version_probe() {
    let root = std::env::temp_dir().join(format!("engines-zit-{}-shellcheck-unlisted", std::process::id()));
    fs::create_dir(&root).unwrap();
    let repo = fs::canonicalize(root).unwrap();
    git_support::git(&repo, &["init", "-q"], b"");
    fs::write(repo.join("quote.sh"), "#!/bin/sh\necho $1\n").unwrap();
    let file = RuleFile::parse(r#"version = 1
[[rule]]
id = "quote"
engine = "shellcheck"
check = "SC9999"
fix = "f"
"#, Path::new(".")).unwrap();
    let probes = Cell::new(0);
    let run = |cmd: &Cmd| {
        if cmd.program != "shellcheck" || cmd.args != vec!["--version"] {
            panic!("unexpected command: {cmd:?}");
        }
        probes.set(probes.get() + 1);
        Ok(Output { code: Some(0), stdout: include_bytes!("../../shellcheck/fixtures/version.txt").to_vec(), stderr: vec![] })
    };
    let host = Host { run: &run, on_path: &|program| program == "shellcheck", raw_chars: &Cell::new(None), base: None };
    let got = run_rules_versioned_with(&file, &repo, &[], &host);
    fs::remove_dir_all(&repo).unwrap();
    assert_eq!(probes.get(), 1);
    assert_eq!(got, CheckedRules {
        runs: vec![RuleRun::new("quote".into(), Engine::ShellCheck, RuleOutcome::NotAvailable {
            rule: "quote".into(),
            engine: Engine::ShellCheck,
            why: "shellcheck 0.11.0: SC9999 is not in temperare's list of shellcheck codes (taken from shellcheck 0.11.0)".into(),
        }, None).with_cause(NotAvailableCause::Other)],
        engines: vec![EngineVersion { engine: Engine::ShellCheck, version: Some("0.11.0".into()) }],
    });
}

#[test]
fn a_marked_rule_with_no_base_still_probes_its_engine_and_runs_nothing_else() {
    let file = RuleFile::parse(r#"version = 1
[[rule]]
id = "base-only"
engine = "ruff"
check = "E711"
only_with_base = true
fix = "f"
"#, Path::new(".")).unwrap();
    let commands = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| {
        let args: Vec<String> = cmd.args.iter().map(|arg| arg.to_string_lossy().into_owned()).collect();
        commands.borrow_mut().push(format!("{} {}", cmd.program.to_string_lossy(), args.join(" ")));
        output(0, "ruff 0.17.4\n", "")
    };
    let host = Host { run: &run, on_path: &|program| program == "ruff", raw_chars: &Cell::new(None), base: None };
    let got = run_rules_versioned_with(&file, Path::new("/"), &[], &host);
    assert_eq!(got, CheckedRules {
        runs: vec![RuleRun::new("base-only".into(), Engine::Ruff, RuleOutcome::NotAvailable {
            rule: "base-only".into(),
            engine: Engine::Ruff,
            why: "reports only what a change adds; run with --base".into(),
        }, None).with_cause(NotAvailableCause::Other)],
        engines: vec![EngineVersion { engine: Engine::Ruff, version: Some("0.17.4".into()) }],
    });
    assert_eq!(commands.into_inner(), ["ruff --version"]);
}
