//! ZUT for run_group.rs: ruff rules of one group sharing one fake check run, a rule marked `only_with_base` with no
//! base kept out of it. The host is fake, so no engine runs,
//! and the ruff flow reads no file, so the rule file's folder is a path that need not exist.

use crate::host::Host;
use crate::run_group::run_group;
use crate::types::{EngineVersion, NotAvailableCause, RuleRun, Stop};
use procio::{Cmd, Output, ProcError};
use rules::{Engine, Finding, Kind, RelPath, Rule, RuleFile, RuleOutcome, Severity};
use std::cell::{Cell, RefCell};
use std::path::Path;

const FOLDER: &str = "/engines-zut-run-group";

const RULES: &str = r#"version = 1
owner = "repo"
[[rule]]
id = "stopped"
engine = "ruff"
check = "B006"
fix = "s"
[[rule]]
id = "none-compare"
engine = "ruff"
check = "E711"
severity = "warning"
fix = "f"
[[rule]]
id = "unused"
engine = "ruff"
check = "F401"
severity = "warning"
paths = ["src/**"]
fix = "g"
[[rule]]
id = "newer"
engine = "ruff"
check = "ZZZ999"
fix = "n"
[[rule]]
id = "base-only"
engine = "ruff"
check = "E711"
only_with_base = true
fix = "b"
"#;

/// 262 characters: an E711 hit in a.py, F401 hits in a.py and src/b.py.
const BOTH: &str = r#"[{"code":"E711","filename":"a.py","location":{"row":1,"column":4},"message":"none"},{"code":"F401","filename":"a.py","location":{"row":2,"column":1},"message":"unused a"},{"code":"F401","filename":"src/b.py","location":{"row":3,"column":8},"message":"unused b"}]"#;

/// 84 characters: the E711 hit alone.
const E711: &str = r#"[{"code":"E711","filename":"a.py","location":{"row":1,"column":4},"message":"none"}]"#;

/// 77 characters: ruff 0.17.4's words for an unknown selector in `--select`.
const UNKNOWN: &str = "ruff failed\n  Cause: Unknown rule selector `ZZZ999` in `select` from the CLI\n";

/// The rules of `RULES` with the given ids, in that order, and the rule file they come from.
fn group(ids: &[&str]) -> (RuleFile, Vec<Rule>) {
    let file = RuleFile::parse(RULES, Path::new(".")).unwrap();
    let rules = ids.iter().map(|id| file.rules.iter().find(|rule| rule.id == *id).unwrap().clone()).collect();
    (file, rules)
}

fn ok(code: i32, stdout: &str, stderr: &str) -> Result<Output, ProcError> {
    Ok(Output { code: Some(code), stdout: stdout.as_bytes().to_vec(), stderr: stderr.as_bytes().to_vec() })
}

/// Each command's `--select=` argument, in the order the fake host was asked to run them.
fn selects(cmd: &Cmd) -> String {
    let select = cmd.args.iter().map(|arg| arg.to_string_lossy()).find(|arg| arg.starts_with("--select="));
    select.unwrap_or_else(|| panic!("not a check run: {cmd:?}")).into_owned()
}

/// `run_group` over `ids` with `run` as the host's run and ruff on PATH when `ruff` is true.
fn grouped(ids: &[&str], run: &dyn Fn(&Cmd) -> Result<Output, ProcError>, ruff: bool) -> Vec<RuleRun> {
    let (file, rules) = group(ids);
    let rules: Vec<&Rule> = rules.iter().collect();
    let on_path = move |program: &str| ruff && program == "ruff";
    let host = Host { run, on_path: &on_path, raw_chars: &Cell::new(Some(999)), base: None };
    let versions = [EngineVersion { engine: Engine::Ruff, version: Some("0.17.4".into()) }];
    run_group(&file, &rules, Ok(Path::new(FOLDER)), &[], &host, &versions, &|rule: &Rule, folder: Result<&Path, &Stop>, host: &Host| crate::engine_hits::engine_hits(rule, folder, host, &[]))
}

fn finding(rule: &str, path: &str, (line, col): (u32, u32), what: &str, fix: &str) -> Finding {
    let mut finding =
        Finding::new(rule, (Engine::Ruff, Kind::Quality, Severity::Warning), RelPath::new(path), line, Some(col), what, fix);
    finding.owner = Some("repo".into());
    finding
}

fn rule_run(rule: &str, outcome: RuleOutcome, raw_chars: Option<usize>) -> RuleRun {
    RuleRun::new(rule.into(), Engine::Ruff, outcome, raw_chars)
}

#[test]
fn one_run_answers_each_rule_with_its_own_codes_and_paths_and_counts_once() {
    let ran = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| {
        ran.borrow_mut().push(selects(cmd));
        ok(1, BOTH, "")
    };
    let got = grouped(&["none-compare", "unused"], &run, true);
    assert_eq!(got, vec![
        rule_run("none-compare", RuleOutcome::Findings(vec![finding("none-compare", "a.py", (1, 4), "none", "f")]), Some(262)),
        rule_run("unused", RuleOutcome::Findings(vec![finding("unused", "src/b.py", (3, 8), "unused b", "g")]), None),
    ]);
    assert_eq!(ran.into_inner(), vec!["--select=E711,F401"]);
}

#[test]
fn a_run_not_found_makes_every_rule_not_available_uncounted() {
    let runs = RefCell::new(0);
    let run = |cmd: &Cmd| -> Result<Output, ProcError> {
        *runs.borrow_mut() += 1;
        Err(ProcError::NotFound { program: cmd.program.clone() })
    };
    let got = grouped(&["none-compare", "unused"], &run, true);
    let absent = |rule: &str| RuleOutcome::NotAvailable { rule: rule.into(), engine: Engine::Ruff, why: "ruff not found on PATH".into() };
    assert_eq!(got, vec![
        rule_run("none-compare", absent("none-compare"), None).with_cause(NotAvailableCause::EngineNotOnPath),
        rule_run("unused", absent("unused"), None).with_cause(NotAvailableCause::EngineNotOnPath),
    ]);
    assert_eq!(runs.into_inner(), 1);
}

#[test]
fn output_no_reader_can_read_fails_every_rule() {
    let runs = RefCell::new(0);
    let run = |_: &Cmd| {
        *runs.borrow_mut() += 1;
        Ok(Output { code: Some(2), stdout: b"bad\xff".to_vec(), stderr: b"oops".to_vec() })
    };
    let got = grouped(&["none-compare", "unused"], &run, true);
    let failed = |rule: &str| RuleOutcome::Failed { rule: rule.into(), engine: Engine::Ruff, code: Some(2), stderr: "oops".into() };
    assert_eq!(got, vec![
        rule_run("none-compare", failed("none-compare"), Some(8)),
        rule_run("unused", failed("unused"), None),
    ]);
    assert_eq!(runs.into_inner(), 1);
}

#[test]
fn an_unknown_check_falls_on_its_rule_and_the_rest_run_again_without_it() {
    let ran = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| {
        let select = selects(cmd);
        ran.borrow_mut().push(select.clone());
        match select.as_str() {
            "--select=E711,ZZZ999" => ok(2, "", UNKNOWN),
            "--select=E711" => ok(1, E711, ""),
            other => panic!("unexpected check run: {other}"),
        }
    };
    let got = grouped(&["none-compare", "newer"], &run, true);
    let why = "ruff 0.17.4 does not know the check `ZZZ999`: Cause: Unknown rule selector `ZZZ999` in `select` from the CLI";
    assert_eq!(got, vec![
        rule_run("none-compare", RuleOutcome::Findings(vec![finding("none-compare", "a.py", (1, 4), "none", "f")]), Some(84)),
        rule_run("newer", RuleOutcome::NotAvailable { rule: "newer".into(), engine: Engine::Ruff, why: why.into() }, Some(77))
            .with_cause(NotAvailableCause::UnknownCheck),
    ]);
    assert_eq!(ran.into_inner(), vec!["--select=E711,ZZZ999", "--select=E711"]);
}

/// A rule whose flow stops before its check run (here a `hits` that fails `stopped` and gives the rest to
/// `engine_hits`) keeps its `Failed` with no count, its check stays out of the shared run, and the run's count goes on
/// the first rule that ran.
#[test]
fn a_rule_stopped_before_its_run_stays_out_of_it_and_the_count_goes_on_the_first_rule_run() {
    let ran = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| {
        ran.borrow_mut().push(selects(cmd));
        ok(1, BOTH, "")
    };
    let hits = |rule: &Rule, folder: Result<&Path, &Stop>, host: &Host| match rule.id.as_str() {
        "stopped" => Err(Stop::Failed { code: None, stderr: "stopped before its run".into() }),
        _ => crate::engine_hits::engine_hits(rule, folder, host, &[]),
    };
    let (file, rules) = group(&["stopped", "none-compare", "unused"]);
    let rules: Vec<&Rule> = rules.iter().collect();
    let host = Host { run: &run, on_path: &|program| program == "ruff", raw_chars: &Cell::new(Some(999)), base: None };
    let versions = [EngineVersion { engine: Engine::Ruff, version: Some("0.17.4".into()) }];
    let got = run_group(&file, &rules, Ok(Path::new(FOLDER)), &[], &host, &versions, &hits);
    let stopped = RuleOutcome::Failed { rule: "stopped".into(), engine: Engine::Ruff, code: None, stderr: "stopped before its run".into() };
    assert_eq!(got, vec![
        rule_run("stopped", stopped, None),
        rule_run("none-compare", RuleOutcome::Findings(vec![finding("none-compare", "a.py", (1, 4), "none", "f")]), Some(262)),
        rule_run("unused", RuleOutcome::Findings(vec![finding("unused", "src/b.py", (3, 8), "unused b", "g")]), None),
    ]);
    assert_eq!(ran.into_inner(), vec!["--select=E711,F401"]);
}

#[test]
fn an_engine_not_available_runs_nothing() {
    let run = |cmd: &Cmd| -> Result<Output, ProcError> { panic!("nothing may run: {cmd:?}") };
    let got = grouped(&["none-compare", "unused"], &run, false);
    let absent = |rule: &str| RuleOutcome::NotAvailable { rule: rule.into(), engine: Engine::Ruff, why: "ruff not found on PATH".into() };
    assert_eq!(got, vec![
        rule_run("none-compare", absent("none-compare"), None).with_cause(NotAvailableCause::EngineNotOnPath),
        rule_run("unused", absent("unused"), None).with_cause(NotAvailableCause::EngineNotOnPath),
    ]);
}

/// `shares_with` ignores `only_with_base`, so the marked E711 rule shares a group with the unmarked F401 rule; with no
/// base it stops before its check run, which then selects F401 alone.
#[test]
fn a_rule_marked_only_with_base_with_no_base_stays_out_of_the_shared_run() {
    let ran = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| {
        ran.borrow_mut().push(cmd.args.iter().map(|arg| arg.to_string_lossy().into_owned()).collect::<Vec<_>>());
        ok(1, BOTH, "")
    };
    let got = grouped(&["base-only", "unused"], &run, true);
    let why = "reports only what a change adds; run with --base";
    assert_eq!(got, vec![
        rule_run("base-only", RuleOutcome::NotAvailable { rule: "base-only".into(), engine: Engine::Ruff, why: why.into() }, None)
            .with_cause(NotAvailableCause::Other),
        rule_run("unused", RuleOutcome::Findings(vec![finding("unused", "src/b.py", (3, 8), "unused b", "g")]), Some(262)),
    ]);
    assert_eq!(ran.into_inner(), vec![vec!["check", "--no-cache", "--output-format", "json", "--select=F401", "."]]);
}
