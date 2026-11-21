//! Unit tests for run_shellcheck_with.rs with a test list of codes, a fake lister of scripts and a fake host: each step's
//! stop, what runs and what is listed before it, the scripts and config reaching the check run, the count taken once
//! from the check run. No engine or git runs. Also `entry`, `rule`, `answers`, `run` and `listed`, which
//! run_shellcheck_tests.rs builds on.
//!
//! The fixtures are real output: version.txt is `shellcheck --version` (0.11.0); sc2086.json is a check run on
//! quote.sh (see parse_shellcheck_tests.rs), 862 characters.

use super::{parse_shellcheck, run_shellcheck_with, shellcheck_refusal, shellcheck_version};
use crate::entry::{Entry, ParamArg, Params, Pick, VersionProbe};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::{Engine, RelPath, Rule, RuleFile};
use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

type Got = Result<Vec<Hit>, Stop>;

const FOLDER: &str = "/no/such/engines-zut-folder";
const LIST: &str = "# source: test\n# version: 9.9.9\nSC2086\nSC2250\n";

/// A shellcheck entry taking `params`. The rule file has no shellcheck engine yet, so it stands under ruff's.
pub(super) fn entry(params: Params) -> Entry {
    Entry {
        name: "shellcheck",
        engine: Engine::Ruff,
        program: "shellcheck",
        needs: &["shellcheck"],
        never: None,
        args: &[],
        pick: Pick::None,
        params,
        config: None,
        after: &[],
        version: Some(VersionProbe { args: &["--version"], read: shellcheck_version }),
        refuse: shellcheck_refusal,
        unknown: |_: &Output, _: &str| None,
        read: parse_shellcheck,
        run: |_, _, _, _| Ok(Vec::new()),
    }
}

pub(super) fn rule(check: &str, extra: &str) -> Rule {
    let text = format!("version = 1\n[[rule]]\nid = \"r\"\nengine = \"ruff\"\ncheck = \"{check}\"\nfix = \"f\"\n{extra}");
    RuleFile::parse(&text, Path::new(FOLDER)).unwrap().rules.remove(0)
}

fn answer(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

/// What the fake host answers: `shellcheck --version` to the probe, any other run to the check run.
pub(super) struct Answers {
    pub(super) probe: Output,
    pub(super) check: Output,
}

pub(super) fn answers() -> Answers {
    Answers {
        probe: answer(0, include_bytes!("fixtures/version.txt"), b""),
        check: answer(1, include_bytes!("fixtures/sc2086.json"), b""),
    }
}

/// `flow(entry, rule, folder, host)` with the host answering `answers`; also the commands run, in order, and the
/// count left in `raw_chars`.
pub(super) fn run(
    flow: impl Fn(&Entry, &Rule, &Path, &Host) -> Result<Vec<Hit>, Stop>,
    entry: &Entry,
    rule: &Rule,
    answers: &Answers,
) -> (Result<Vec<Hit>, Stop>, Vec<Cmd>, Option<usize>) {
    let ran = RefCell::new(Vec::new());
    let host_run = |cmd: &Cmd| -> Result<Output, ProcError> {
        ran.borrow_mut().push(cmd.clone());
        Ok(match (cmd.program.to_str(), cmd.args.as_slice()) {
            (Some("shellcheck"), [arg]) if arg == "--version" => answers.probe.clone(),
            _ => answers.check.clone(),
        })
    };
    let raw_chars = Cell::new(None);
    let got = flow(entry, rule, Path::new(FOLDER), &Host { run: &host_run, on_path: &|_| true, raw_chars: &raw_chars });
    (got, ran.into_inner(), raw_chars.get())
}

/// `run_shellcheck_with` on `codes` with the lister giving `scripts`, through `run`; also the folders the lister was
/// asked for.
pub(super) fn listed(
    codes: &str,
    scripts: Result<Vec<PathBuf>, Stop>,
    rule: &Rule,
    answers: &Answers,
) -> (Got, Vec<Cmd>, Option<usize>, Vec<PathBuf>) {
    let asked = RefCell::new(Vec::new());
    let list = |folder: &Path| {
        asked.borrow_mut().push(folder.to_path_buf());
        scripts.clone()
    };
    let flow = |entry: &Entry, rule: &Rule, folder: &Path, host: &Host| run_shellcheck_with(codes, &list, entry, rule, folder, host);
    let (got, ran, raw_chars) = run(flow, &entry(Params::Args(&[])), rule, answers);
    (got, ran, raw_chars, asked.into_inner())
}

fn scripts() -> Result<Vec<PathBuf>, Stop> {
    Ok(vec![PathBuf::from("a.sh"), PathBuf::from("b.sh")])
}

fn with_list(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Got {
    run_shellcheck_with(LIST, &|_| scripts(), entry, rule, folder, host)
}

fn programs(ran: &[Cmd]) -> Vec<OsString> {
    ran.iter().map(|cmd| cmd.program.clone()).collect()
}

fn os(args: &[&str]) -> Vec<OsString> {
    args.iter().map(OsString::from).collect()
}

fn failed(stderr: &str) -> Got {
    Err(Stop::Failed { code: None, stderr: stderr.to_string() })
}

#[test]
fn a_malformed_check_fails_with_the_refusal_and_nothing_runs() {
    let (got, ran, raw_chars) = run(with_list, &entry(Params::Args(&[])), &rule("sc2086", ""), &answers());
    assert_eq!(
        got,
        failed("shellcheck check `sc2086` is not one ShellCheck code such as `SC2086`, so tm cannot tell which findings it selects")
    );
    assert!(ran.is_empty(), "{ran:?}");
    assert_eq!(raw_chars, None);
}

#[test]
fn a_param_fails_and_nothing_runs() {
    let (got, ran, _) = run(with_list, &entry(Params::Args(&[])), &rule("SC2086", "params = { x = [\"y\"] }\n"), &answers());
    assert_eq!(got, failed("shellcheck param x: a shellcheck rule takes no params"));
    assert!(ran.is_empty(), "{ran:?}");
}

#[test]
fn a_param_the_entry_takes_still_fails_since_the_run_has_no_place_for_it() {
    static ALLOW: [ParamArg; 1] = [ParamArg { name: "allow", flag: "--allow=" }];
    let (got, ran, _) = run(with_list, &entry(Params::Args(&ALLOW)), &rule("SC2086", "params = { allow = [\"y\"] }\n"), &answers());
    assert_eq!(got, failed("shellcheck passes no params to its run"));
    assert!(ran.is_empty(), "{ran:?}");
}

#[test]
fn a_list_that_cannot_be_read_fails() {
    let (got, ran, _, asked) = listed("SC2086\n", scripts(), &rule("SC2086", ""), &answers());
    assert_eq!(got, failed("temperare's list of shellcheck codes cannot be read: no \"# source:\" line"));
    assert!(ran.is_empty(), "{ran:?}");
    assert!(asked.is_empty(), "{asked:?}");
}

#[test]
fn a_code_not_in_the_list_is_not_available_and_only_the_probe_runs() {
    let (got, ran, raw_chars, asked) = listed(LIST, scripts(), &rule("SC9999", ""), &answers());
    assert!(asked.is_empty(), "{asked:?}");
    let why = "shellcheck 0.11.0: SC9999 is not in temperare's list of shellcheck codes (taken from shellcheck 9.9.9)";
    assert_eq!(got, Err(Stop::NotAvailable { why: why.to_string() }));
    assert_eq!(ran.len(), 1);
    assert_eq!(ran[0].program, OsString::from("shellcheck"));
    assert_eq!(ran[0].args, os(&["--version"]));
    assert_eq!(raw_chars, None);
}

#[test]
fn a_leading_zero_code_is_not_in_the_list_so_it_is_not_available_and_only_the_probe_runs() {
    let (got, ran, raw_chars) = run(with_list, &entry(Params::Args(&[])), &rule("SC02086", ""), &answers());
    let why = "shellcheck 0.11.0: SC02086 is not in temperare's list of shellcheck codes (taken from shellcheck 9.9.9)";
    assert_eq!(got, Err(Stop::NotAvailable { why: why.to_string() }));
    assert_eq!(ran.len(), 1);
    assert_eq!(ran[0].program, OsString::from("shellcheck"));
    assert_eq!(ran[0].args, os(&["--version"]));
    assert_eq!(raw_chars, None);
}

#[test]
fn a_code_not_in_the_list_with_a_failed_probe_has_its_version_unknown() {
    let answers = Answers { probe: answer(2, b"", b"boom"), ..answers() };
    let (got, _, _) = run(with_list, &entry(Params::Args(&[])), &rule("SC9999", ""), &answers);
    let why = "shellcheck (version unknown): SC9999 is not in temperare's list of shellcheck codes (taken from shellcheck 9.9.9)";
    assert_eq!(got, Err(Stop::NotAvailable { why: why.to_string() }));
}

#[test]
fn no_shell_files_is_no_hits_and_nothing_runs() {
    let (got, ran, raw_chars, asked) = listed(LIST, Ok(vec![]), &rule("SC2086", ""), &answers());
    assert_eq!(got, Ok(vec![]));
    assert_eq!(asked, vec![PathBuf::from(FOLDER)]);
    assert!(ran.is_empty(), "{ran:?}");
    assert_eq!(raw_chars, None);
}

#[test]
fn the_scripts_reach_the_check_run_which_is_read_and_counted_once() {
    let (got, ran, raw_chars, asked) = listed(LIST, scripts(), &rule("SC2086", ""), &answers());
    assert_eq!(asked, vec![PathBuf::from(FOLDER)]);
    assert_eq!(programs(&ran), os(&["shellcheck"]));
    assert_eq!(ran[0].args, os(&["-f", "json1", "--enable=all", "--norc", "--", "a.sh", "b.sh"]));
    assert_eq!(ran[0].cwd, Some(PathBuf::from(FOLDER)));
    let hit = Hit {
        path: RelPath::new("quote.sh"),
        line: Some(3),
        col: Some(6),
        what: "Double quote to prevent globbing and word splitting.".to_string(),
    };
    assert_eq!(got, Ok(vec![hit]));
    assert_eq!(raw_chars, Some(862));
}

#[test]
fn the_rules_config_is_the_rcfile() {
    let (_, ran, _) = run(with_list, &entry(Params::Args(&[])), &rule("SC2086", "config = \"x.rc\"\n"), &answers());
    assert_eq!(ran[0].args, os(&["-f", "json1", "--enable=all", "--rcfile=x.rc", "--", "a.sh", "b.sh"]));
}

#[test]
fn a_failed_listing_stops_before_the_check_run() {
    let stop = Stop::Failed { code: Some(128), stderr: "git ls-files: exit 128: fatal: x".to_string() };
    let (got, ran, raw_chars, _) = listed(LIST, Err(stop), &rule("SC2086", ""), &answers());
    assert_eq!(got, Err(Stop::Failed { code: Some(128), stderr: "git ls-files: exit 128: fatal: x".to_string() }));
    assert!(ran.is_empty(), "{ran:?}");
    assert_eq!(raw_chars, None);
}

#[test]
fn a_check_run_that_is_not_json_is_the_parse_failure() {
    let answers = Answers { check: answer(1, b"not json", b""), ..answers() };
    let (got, _, _) = run(with_list, &entry(Params::Args(&[])), &rule("SC2086", ""), &answers);
    assert_eq!(got, Err(Stop::Failed { code: Some(1), stderr: "shellcheck printed no JSON findings: not json".to_string() }));
}
