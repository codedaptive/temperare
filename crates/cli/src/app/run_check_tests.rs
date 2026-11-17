//! ZUT for run_check.rs, with closure worlds: no git, no disk, no PATH. Every run is in /repo with the rule file
//! /repo/temperare.toml parsed from literal TOML; a seam a test must not reach panics.

use super::{run_check, CheckWorld};
use crate::CheckArgs;
use procio::{GitError, Oid};
use report::{Counts, NotAvailable, Report, ReportError};
use rules::{Engine, Finding, Kind, Problem, RelPath, RuleFile, RuleOutcome, Severity};
use serde_json::json;
use std::cell::{Cell, RefCell};
use std::path::{Path, PathBuf};

const TEXT: &str = "version = 1
owner = \"core\"
[[rule]]
id = \"q\"
engine = \"ruff\"
check = \"F401\"
fix = \"Remove the import.\"
[[rule]]
id = \"s\"
engine = \"ast-grep\"
kind = \"security\"
check = \"no-eval\"
fix = \"Do not eval.\"
";

const HEX: &str = "0123456789abcdef0123456789abcdef01234567";

type Outcomes = Vec<(String, RuleOutcome)>;

fn top(_: &Path) -> Result<PathBuf, GitError> {
    Ok(PathBuf::from("/repo"))
}

fn there(_: &Path) -> bool {
    true
}

fn read(path: &Path) -> Result<RuleFile, Vec<Problem>> {
    assert_eq!(path, Path::new("/repo/temperare.toml"));
    Ok(RuleFile::parse(TEXT, Path::new("/repo")).unwrap())
}

fn no_outcomes(_: &RuleFile, _: &Path, _: &[RelPath]) -> Outcomes {
    Vec::new()
}

fn clean(_: &RuleFile, _: Outcomes, _: Option<(&Path, &Oid)>) -> Result<Report, ReportError> {
    Ok(report(vec![], vec![]))
}

/// A world that finds and reads the rule file, runs no rule and reports nothing; `rev_parse` panics.
fn world() -> CheckWorld<'static> {
    CheckWorld {
        toplevel: &top,
        exists: &there,
        read: &read,
        rev_parse: &|_, _| unreachable!("rev_parse"),
        run_rules: &no_outcomes,
        build_report: &clean,
    }
}

fn report(findings: Vec<Finding>, not_available: Vec<NotAvailable>) -> Report {
    Report {
        findings,
        silenced: vec![],
        exceptions_in_force: 0,
        not_available,
        failed: vec![],
        base: None,
        counts: Counts {
            findings: 0,
            errors: 0,
            warnings: 0,
            quality: 0,
            security: 0,
            silenced: 0,
            not_available: 0,
            failed: 0,
        },
    }
}

fn quality_error() -> Finding {
    Finding {
        rule: "q".to_string(),
        engine: Engine::Ruff,
        kind: Kind::Quality,
        severity: Severity::Error,
        owner: Some("core".to_string()),
        path: RelPath::new("a.py"),
        line: 7,
        col: Some(3),
        what: "unused import".to_string(),
        fix: "Remove the import.".to_string(),
    }
}

/// Runs `tm check` in /repo; (exit, out, err).
fn check(args: &CheckArgs, world: &CheckWorld) -> (u8, String, String) {
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let code = run_check(args, Path::new("/repo"), world, &mut out, &mut err);
    (code, String::from_utf8(out).unwrap(), String::from_utf8(err).unwrap())
}

/// As text: exit 2, nothing on stdout, `stderr` on stderr. Under --json: exit 2, the same stderr, `object` on stdout.
fn assert_usage_error(args: CheckArgs, world: &CheckWorld, stderr: &str, object: serde_json::Value) {
    assert_eq!(check(&args, world), (2, String::new(), stderr.to_string()));
    let (code, out, err) = check(&CheckArgs { json: true, ..args }, world);
    assert_eq!((code, err.as_str()), (2, stderr));
    assert!(out.ends_with("}\n"), "{out}");
    assert_eq!(serde_json::from_str::<serde_json::Value>(&out).unwrap(), object);
}

#[test]
fn outside_a_repository_is_a_usage_error() {
    let w = CheckWorld {
        toplevel: &|cwd| Err(GitError::NotARepository { repo: cwd.to_path_buf() }),
        read: &|_| unreachable!("read"),
        run_rules: &|_, _, _| unreachable!("run_rules"),
        ..world()
    };
    assert_usage_error(
        CheckArgs::default(),
        &w,
        "tm: not in a git repository: /repo\nnext: Run tm inside a git repository, or pass --rules <path>.\n",
        json!({
            "error": "not in a git repository: /repo",
            "exit": 2,
            "next": "Run tm inside a git repository, or pass --rules <path>.",
        }),
    );
}

#[test]
fn rule_file_problems_are_listed_with_exit_2() {
    let broken = |_: &Path| {
        let what = "version must be 1".to_string();
        Err(vec![Problem { line: 1, col: 11, what, fix: "Write version = 1.".to_string() }])
    };
    let w = CheckWorld { read: &broken, run_rules: &|_, _, _| unreachable!("run_rules"), ..world() };
    assert_eq!(
        check(&CheckArgs::default(), &w),
        (
            2,
            "/repo/temperare.toml:1:11  version must be 1  Write version = 1.\n\
             next: fix each problem listed for /repo/temperare.toml\n"
                .to_string(),
            String::new()
        )
    );
    let (code, out, err) = check(&CheckArgs { json: true, ..CheckArgs::default() }, &w);
    assert_eq!((code, err.as_str()), (2, ""));
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&out).unwrap(),
        json!({
            "rule_file": "/repo/temperare.toml",
            "problems": [{"line": 1, "col": 11, "what": "version must be 1", "fix": "Write version = 1."}],
            "exit": 2,
            "next": "fix each problem listed for /repo/temperare.toml",
        })
    );
}

#[test]
fn a_path_outside_the_rule_files_folder_is_a_usage_error() {
    let w = CheckWorld { run_rules: &|_, _, _| unreachable!("run_rules"), ..world() };
    let args = CheckArgs { paths: vec![PathBuf::from("/elsewhere/x.py")], ..CheckArgs::default() };
    assert_usage_error(
        args,
        &w,
        "tm: /elsewhere/x.py is not under the rule file's folder /repo\n\
         next: Pass paths inside /repo, or pass --rules with the rule file that covers them.\n",
        json!({
            "error": "/elsewhere/x.py is not under the rule file's folder /repo",
            "exit": 2,
            "next": "Pass paths inside /repo, or pass --rules with the rule file that covers them.",
        }),
    );
}

#[test]
fn a_base_naming_no_commit_is_a_usage_error_before_any_engine_runs() {
    let w = CheckWorld { rev_parse: &|_, _| Ok(None), run_rules: &|_, _, _| unreachable!("run_rules"), ..world() };
    let args = CheckArgs { base: Some("nosuchrev".to_string()), ..CheckArgs::default() };
    assert_usage_error(
        args,
        &w,
        "tm: base 'nosuchrev' does not name a commit\n\
         next: Pass a commit, branch or tag that exists, e.g. --base main.\n",
        json!({
            "error": "base 'nosuchrev' does not name a commit",
            "exit": 2,
            "next": "Pass a commit, branch or tag that exists, e.g. --base main.",
        }),
    );
}

#[test]
fn a_report_that_cannot_be_built_is_a_usage_error() {
    let fails = |_: &RuleFile, _: Outcomes, _: Option<(&Path, &Oid)>| {
        Err(ReportError::Folder { folder: PathBuf::from("/repo"), why: "not inside /top".to_string() })
    };
    let w = CheckWorld { build_report: &fails, ..world() };
    assert_usage_error(
        CheckArgs::default(),
        &w,
        "tm: cannot build the report: rule file folder /repo: not inside /top\n\
         next: Check that the rule file's folder is inside the repository and git runs here.\n",
        json!({
            "error": "cannot build the report: rule file folder /repo: not inside /top",
            "exit": 2,
            "next": "Check that the rule file's folder is inside the repository and git runs here.",
        }),
    );
}

#[test]
fn kind_security_passes_only_the_security_rules_to_run_rules() {
    let ids = RefCell::new(Vec::new());
    let run_rules = |rf: &RuleFile, _: &Path, _: &[RelPath]| {
        ids.borrow_mut().extend(rf.rules.iter().map(|r| r.id.clone()));
        Vec::new()
    };
    let w = CheckWorld { run_rules: &run_rules, ..world() };
    let (code, _, _) = check(&CheckArgs { kind: Some(Kind::Security), ..CheckArgs::default() }, &w);
    assert_eq!(code, 0);
    assert_eq!(*ids.borrow(), vec!["s".to_string()]);
}

#[test]
fn paths_reach_run_rules_as_paths_under_the_folder_and_outcomes_reach_the_report() {
    let seen = RefCell::new(None);
    let run_rules = |_: &RuleFile, repo: &Path, only: &[RelPath]| {
        *seen.borrow_mut() = Some((repo.to_path_buf(), only.to_vec()));
        vec![("q".to_string(), RuleOutcome::Findings(vec![]))]
    };
    let build_report = |_: &RuleFile, outcomes: Outcomes, _: Option<(&Path, &Oid)>| {
        assert_eq!(outcomes, vec![("q".to_string(), RuleOutcome::Findings(vec![]))]);
        Ok(report(vec![], vec![]))
    };
    let w = CheckWorld { run_rules: &run_rules, build_report: &build_report, ..world() };
    let args = CheckArgs { paths: vec![PathBuf::from("src/a.py"), PathBuf::from("./b.py")], ..CheckArgs::default() };
    assert_eq!(check(&args, &w).0, 0);
    assert_eq!(
        *seen.borrow(),
        Some((PathBuf::from("/repo"), vec![RelPath::new("src/a.py"), RelPath::new("b.py")]))
    );
}

#[test]
fn no_base_asks_git_only_for_the_rule_file_and_reports_without_a_base() {
    let tops = Cell::new(0);
    let toplevel = |_: &Path| {
        tops.set(tops.get() + 1);
        Ok(PathBuf::from("/repo"))
    };
    let bases = RefCell::new(Vec::new());
    let build_report = |_: &RuleFile, _: Outcomes, base: Option<(&Path, &Oid)>| {
        bases.borrow_mut().push(base.map(|(top, oid)| (top.to_path_buf(), oid.clone())));
        Ok(report(vec![], vec![]))
    };
    let w = CheckWorld { toplevel: &toplevel, build_report: &build_report, ..world() };
    assert_eq!(check(&CheckArgs::default(), &w).0, 0);
    assert_eq!(tops.get(), 1);
    assert_eq!(*bases.borrow(), vec![None]);
}

#[test]
fn a_resolved_base_reaches_the_report_as_the_top_and_the_commit() {
    let rev_parse = |top: &Path, rev: &str| {
        assert_eq!((top, rev), (Path::new("/repo"), "main"));
        Ok(Oid::parse(HEX))
    };
    let bases = RefCell::new(Vec::new());
    let build_report = |_: &RuleFile, _: Outcomes, base: Option<(&Path, &Oid)>| {
        bases.borrow_mut().push(base.map(|(top, oid)| (top.to_path_buf(), oid.as_str().to_string())));
        Ok(report(vec![], vec![]))
    };
    let w = CheckWorld { rev_parse: &rev_parse, build_report: &build_report, ..world() };
    assert_eq!(check(&CheckArgs { base: Some("main".to_string()), ..CheckArgs::default() }, &w).0, 0);
    assert_eq!(*bases.borrow(), vec![Some((PathBuf::from("/repo"), HEX.to_string()))]);
}

#[test]
fn a_clean_report_is_the_counts_then_nothing_to_do() {
    assert_eq!(
        check(&CheckArgs::default(), &world()),
        (
            0,
            "0 findings (0 error, 0 warning; 0 security), 0 silenced, 0 exceptions in force, 0 not available, \
             0 failed\nnext: nothing to do\n"
                .to_string(),
            String::new()
        )
    );
}

#[test]
fn a_quality_error_exits_1_and_names_the_rule_file() {
    let found = |_: &RuleFile, _: Outcomes, _: Option<(&Path, &Oid)>| Ok(report(vec![quality_error()], vec![]));
    let (code, out, err) = check(&CheckArgs::default(), &CheckWorld { build_report: &found, ..world() });
    assert_eq!((code, err.as_str()), (1, ""));
    assert!(out.starts_with("a.py:7:3  error quality q  owner core\n"), "{out}");
    assert!(
        out.ends_with("next: fix the findings above, or add an approved [[exception]] to /repo/temperare.toml\n"),
        "{out}"
    );
}

#[test]
fn an_absent_engine_exits_3_and_names_it() {
    let absent = |_: &RuleFile, _: Outcomes, _: Option<(&Path, &Oid)>| {
        let rule = |id: &str| NotAvailable {
            rule: id.to_string(),
            engine: Engine::SwiftLint,
            why: "swiftlint not found on PATH".to_string(),
        };
        Ok(report(vec![], vec![rule("s1"), rule("s2")]))
    };
    let (code, out, err) = check(&CheckArgs::default(), &CheckWorld { build_report: &absent, ..world() });
    assert_eq!((code, err.as_str()), (3, ""));
    assert!(out.ends_with("next: install swiftlint (or run without those rules)\n"), "{out}");
}

#[test]
fn json_is_one_line_with_the_report_exit_and_next_at_the_top() {
    let found = |_: &RuleFile, _: Outcomes, _: Option<(&Path, &Oid)>| Ok(report(vec![quality_error()], vec![]));
    let w = CheckWorld { build_report: &found, ..world() };
    let (code, out, err) = check(&CheckArgs { json: true, ..CheckArgs::default() }, &w);
    assert_eq!((code, err.as_str()), (1, ""));
    assert_eq!(out.lines().count(), 1, "{out}");
    let value: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(value["exit"], 1);
    assert_eq!(value["next"], "fix the findings above, or add an approved [[exception]] to /repo/temperare.toml");
    assert_eq!(value["findings"][0]["rule"], "q");
    assert_eq!(value["findings"][0]["path"], "a.py");
    assert!(value["counts"].is_object(), "{out}");
    assert_eq!(value["exceptions_in_force"], 0);
}
