//! ZUT for run_rules_listed.rs: rules flagged by a listing of the folder's files made by hand, with a fake host. The host
//! panics on any run or PATH lookup it must not see, so a flagged rule is shown to run nothing and look nothing up.
//! The ruff flow reads no file, so the rule file's folder is a path that need not exist; an ast-grep rule refused for its
//! count param is settled before its engine is looked up, so it reads none either.

use crate::entry::entry_cmd;
use crate::entry_for::entry_for;
use crate::host::Host;
use crate::run_rules_listed::run_rules_listed;
use crate::types::{RuleRun, Stop};
use procio::{Cmd, Output, ProcError};
use rules::{Engine, Finding, Kind, RelPath, Rule, RuleFile, RuleOutcome, Severity};
use std::cell::{Cell, RefCell};
use std::ffi::OsStr;
use std::path::Path;
use std::rc::Rc;

const FOLDER: &str = "/engines-zut-run-rules-listed";

/// Three ruff rules that pairwise share a run; `elsewhere`'s paths match no file of `files()`.
const RULES: &str = r#"version = 1
owner = "repo"
[[rule]]
id = "first"
engine = "ruff"
check = "E711"
paths = ["src/**"]
fix = "f"
[[rule]]
id = "elsewhere"
engine = "ruff"
check = "F401"
paths = ["lib/**"]
fix = "g"
[[rule]]
id = "third"
engine = "ruff"
check = "E501"
paths = ["src/**"]
fix = "h"
"#;

/// 88 characters: one E711 hit in src/a.py.
const E711: &str = r#"[{"code":"E711","filename":"src/a.py","location":{"row":1,"column":4},"message":"none"}]"#;

const WHY: &str = "its paths match no file in the rule file's folder";

fn files() -> Vec<RelPath> {
    vec![RelPath::new("src/a.py"), RelPath::new("docs/readme.md")]
}

/// The rule file holding the rules of `RULES` with the given ids, in that order.
fn file(ids: &[&str]) -> RuleFile {
    let mut file = RuleFile::parse(&format!("{RULES}{REFUSED}"), Path::new(".")).unwrap();
    file.rules = ids.iter().map(|id| file.rules.iter().find(|rule| rule.id == *id).unwrap().clone()).collect();
    file
}

fn listed(
    ids: &[&str],
    run: &dyn Fn(&Cmd) -> Result<Output, ProcError>,
    on_path: &dyn Fn(&str) -> bool,
    files: Option<&[RelPath]>,
) -> Vec<RuleRun> {
    let host = Host { run, on_path, raw_chars: &Cell::new(Some(999)), base: None };
    run_rules_listed(&file(ids), Ok(Path::new(FOLDER)), &[], &host, &[], files, &crate::engine_hits::engine_hits)
}

fn ok(code: i32, stdout: &str) -> Result<Output, ProcError> {
    Ok(Output { code: Some(code), stdout: stdout.as_bytes().to_vec(), stderr: Vec::new() })
}

fn no_run(cmd: &Cmd) -> Result<Output, ProcError> {
    panic!("nothing may run: {cmd:?}")
}

fn no_lookup(program: &str) -> bool {
    panic!("nothing may be looked up: {program}")
}

fn ruff_only(program: &str) -> bool {
    program == "ruff"
}

fn flagged(rule: &str) -> RuleRun {
    RuleRun::new(rule.into(), Engine::Ruff, RuleOutcome::Findings(vec![]), None).with_no_files(WHY)
}

fn first_found(raw_chars: Option<usize>) -> RuleRun {
    let mut finding =
        Finding::new("first", (Engine::Ruff, Kind::Quality, Severity::Error), RelPath::new("src/a.py"), 1, Some(4), "none", "f");
    finding.owner = Some("repo".into());
    RuleRun::new("first".into(), Engine::Ruff, RuleOutcome::Findings(vec![finding]), raw_chars)
}

/// The fake `hits` here is the engine_hits seam. Each call records the base
/// its host carries and whether `host.run` answered the rule's own check command: in a shared group the probe pass
/// answers it as a run that could not start, and the run pass answers it with the one shared run.
#[test]
fn the_base_reaches_each_rules_run_alone_and_both_passes_of_a_shared_group() {
    let answered = |base: &str| (Some(base.to_string()), true);
    let probe = |base: &str| (Some(base.to_string()), false);
    let cases: [(&[&str], Vec<(Option<String>, bool)>, Vec<Vec<&str>>); 2] = [
        (&["first"], vec![answered("abc123")], vec![vec!["check", "--no-cache", "--output-format", "json", "--select=E711", "."]]),
        (
            &["first", "third"],
            vec![probe("abc123"), probe("abc123"), answered("abc123"), answered("abc123")],
            vec![vec!["check", "--no-cache", "--output-format", "json", "--select=E711,E501", "."]],
        ),
    ];
    for (ids, want, want_ran) in cases {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let recorded = Rc::clone(&seen);
        let hits = move |rule: &Rule, _: Result<&Path, &Stop>, host: &Host| {
            let cmd = entry_cmd(entry_for(rule.engine).unwrap(), OsStr::new(&rule.check), &[], None, Path::new(FOLDER));
            let ran = (host.run)(&cmd).is_ok();
            recorded.borrow_mut().push((host.base.map(str::to_string), ran));
            Ok(vec![])
        };
        let ran = RefCell::new(Vec::new());
        let run = |cmd: &Cmd| {
            ran.borrow_mut().push(cmd.args.iter().map(|arg| arg.to_string_lossy().into_owned()).collect::<Vec<_>>());
            ok(0, "[]")
        };
        let host = Host { run: &run, on_path: &no_lookup, raw_chars: &Cell::new(None), base: Some("abc123") };
        let got = run_rules_listed(&file(ids), Ok(Path::new(FOLDER)), &[], &host, &[], None, &hits);
        assert_eq!(got.len(), ids.len(), "{ids:?}");
        assert_eq!(*seen.borrow(), want, "{ids:?}");
        assert_eq!(ran.into_inner(), want_ran, "{ids:?}");
    }
}

#[test]
fn the_central_settle_runs_before_every_flow() {
    let cases = [
        ("ruff", Engine::Ruff, "flake8-bugbear", "ruff check `flake8-bugbear`: `flake8-bugbear` is not a rule code, a code prefix or ALL, so tm cannot tell which findings it selects"),
        ("shellcheck", Engine::ShellCheck, "sc2086", "shellcheck check `sc2086` is not one ShellCheck code such as `SC2086`, so tm cannot tell which findings it selects"),
        ("clippy", Engine::Clippy, "rustdoc::broken_intra_doc_links", "clippy check `rustdoc::broken_intra_doc_links` is a rustdoc lint: clippy never runs rustdoc lints, so tm cannot report it"),
        ("ast-grep", Engine::AstGrep, "/abs/names.yml", "ast-grep rule /abs/names.yml is an absolute path: a check names a file inside the rule file's folder"),
        ("git", Engine::Git, "other", "git check `other` is not one git counts: the git entry checks only `test_ratio`"),
    ];
    for (name, engine, check, stderr) in cases {
        let text = format!("version = 1\n[[rule]]\nid = \"r\"\nengine = \"{name}\"\ncheck = \"{check}\"\npaths = [\"src/**\"]\nfix = \"f\"\n");
        let file = RuleFile::parse(&text, Path::new(".")).unwrap();
        let host = Host { run: &no_run, on_path: &|_| true, raw_chars: &Cell::new(None), base: None };
        let hits = |_: &Rule, _: Result<&Path, &Stop>, _: &Host| panic!("no flow may run");
        let files = [RelPath::new("src/a.py")];
        let got = run_rules_listed(&file, Ok(Path::new(FOLDER)), &[], &host, &[], Some(&files), &hits);
        let outcome = RuleOutcome::Failed { rule: "r".into(), engine, code: None, stderr: stderr.into() };
        assert_eq!(got, vec![RuleRun::new("r".into(), engine, outcome, None)], "{name}");
    }
}

#[test]
fn a_rule_matching_no_file_is_flagged_with_nothing_run_or_looked_up() {
    let files = files();
    let got = listed(&["elsewhere"], &no_run, &no_lookup, Some(&files));
    assert_eq!(got, vec![flagged("elsewhere")]);
    assert_eq!(got[0].no_files.as_deref(), Some("its paths match no file in the rule file's folder"));
}

#[test]
fn a_rule_matching_a_file_runs_with_no_flag() {
    let files = files();
    let run = |_: &Cmd| ok(1, E711);
    assert_eq!(listed(&["first"], &run, &ruff_only, Some(&files)), vec![first_found(Some(88))]);
}

#[test]
fn no_listing_flags_nothing_and_every_rule_runs() {
    let run = |_: &Cmd| ok(0, "[]");
    let got = listed(&["elsewhere"], &run, &ruff_only, None);
    assert_eq!(got, vec![RuleRun::new("elsewhere".into(), Engine::Ruff, RuleOutcome::Findings(vec![]), Some(2))]);
    assert_eq!(got[0].no_files, None);
}

#[test]
fn an_empty_listing_flags_every_rule() {
    let got = listed(&["first", "elsewhere"], &no_run, &no_lookup, Some(&[]));
    assert_eq!(got, vec![flagged("first"), flagged("elsewhere")]);
}

#[test]
fn a_flagged_rule_between_two_sharing_rules_stays_out_of_their_shared_run() {
    let files = files();
    let ran = RefCell::new(Vec::new());
    let run = |cmd: &Cmd| {
        let select = cmd.args.iter().map(|arg| arg.to_string_lossy().into_owned()).find(|arg| arg.starts_with("--select="));
        let select = select.unwrap_or_else(|| panic!("not a check run: {cmd:?}"));
        ran.borrow_mut().push(select.clone());
        match select.as_str() {
            "--select=E711,E501" => ok(1, E711),
            other => panic!("unexpected check run: {other}"),
        }
    };
    let got = listed(&["first", "elsewhere", "third"], &run, &ruff_only, Some(&files));
    assert_eq!(got, vec![
        first_found(Some(88)),
        flagged("elsewhere"),
        RuleRun::new("third".into(), Engine::Ruff, RuleOutcome::Findings(vec![]), None),
    ]);
    assert_eq!(ran.into_inner(), vec!["--select=E711,E501"]);
}

/// Two ruff rules tm refuses, each with paths matching no file of `files()`: a check that is not a code, and a param
/// ruff does not take.
const REFUSED: &str = r#"[[rule]]
id = "malformed"
engine = "ruff"
check = "flake8-bugbear"
paths = ["lib/**"]
fix = "m"
[[rule]]
id = "unknown-param"
engine = "ruff"
check = "E711"
paths = ["lib/**"]
params = { nope = ["x"] }
fix = "p"
"#;

#[test]
fn a_refused_rule_is_failed_before_the_no_file_flag_with_nothing_run_or_looked_up() {
    let files = files();
    let malformed = "ruff check `flake8-bugbear`: `flake8-bugbear` is not a rule code, a code prefix or ALL, so tm cannot tell which findings it selects";
    let param = "ruff param nope: a ruff rule reads only the `max_complexity` param";
    let cases: [(&str, Option<&[RelPath]>, &str); 3] = [
        ("malformed", Some(&files), malformed),
        ("unknown-param", Some(&files), param),
        ("malformed", None, malformed),
    ];
    for (id, listing, stderr) in cases {
        let got = listed(&[id], &no_run, &no_lookup, listing);
        let failed = RuleOutcome::Failed { rule: id.into(), engine: Engine::Ruff, code: None, stderr: stderr.into() };
        assert_eq!(got, vec![RuleRun::new(id.into(), Engine::Ruff, failed, None)], "{id}, listing {listing:?}");
        assert_eq!(got[0].no_files, None, "{id}");
    }
}

#[test]
fn a_bad_count_param_with_ast_grep_absent_is_failed_not_not_available() {
    let text = "version = 1\n[[rule]]\nid = \"r\"\nengine = \"ast-grep\"\ncheck = \"names.yml\"\npaths = [\"src/**\"]\nparams = { count = \"wrong\" }\nfix = \"f\"\n";
    let file = RuleFile::parse(text, Path::new(".")).unwrap();
    let host = Host { run: &no_run, on_path: &|_| false, raw_chars: &Cell::new(None), base: Some("abc123") };
    let stderr = "param count in names.yml: takes only \"setup-vs-tests\"";
    let failed = RuleOutcome::Failed { rule: "r".into(), engine: Engine::AstGrep, code: None, stderr: stderr.into() };
    let listings: [Option<&[RelPath]>; 2] = [None, Some(&[])];
    for listing in listings {
        let got = run_rules_listed(&file, Ok(Path::new(FOLDER)), &[], &host, &[], listing, &crate::engine_hits::engine_hits);
        assert_eq!(got, vec![RuleRun::new("r".into(), Engine::AstGrep, failed.clone(), None)], "listing {listing:?}");
    }
}
