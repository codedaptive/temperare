//! ZUT for run_case.rs, with recording closures: no disk, no engine. Every step the case takes on its world is
//! logged, so each row shows what was written, what ran (with the base it was given) and that the folder was
//! removed.

use super::run_case::run_case;
use super::BaselineWorld;
use crate::output::CaseVerdict;
use engines::{CheckedRules, NotAvailableCause, RuleRun};
use rules::{CaseFile, Engine, ExpectedFinding, Finding, Kind, RelPath, Rule, RuleFile, RuleOutcome, Severity};
use std::cell::RefCell;
use std::io;
use std::path::{Path, PathBuf};

/// How the world answers: whether make_folder and remove_folder fail, the file name whose write fails, and the
/// findings the rule's run gives; with `not_available` set, the run is not available (`ruff not found on PATH`) with
/// that cause instead.
struct Setup {
    make_fails: bool,
    write_fails: Option<&'static str>,
    remove_fails: bool,
    findings: Vec<Finding>,
    not_available: Option<NotAvailableCause>,
}

const OK: Setup =
    Setup { make_fails: false, write_fails: None, remove_fails: false, findings: Vec::new(), not_available: None };

fn finding(line: u32) -> Finding {
    Finding::new("tm-zeta", (Engine::Ruff, Kind::Quality, Severity::Error), RelPath::new("a.py"), line, None, "w", "f")
}

fn files(names: &[&str]) -> Vec<CaseFile> {
    names.iter().map(|name| CaseFile { path: name.to_string(), text: "import os\n" }).collect()
}

/// Runs the failing case `files` of rule tm-zeta (expected at a.py:1) in the world `setup` describes; (verdict, the
/// steps taken, what was written on err).
fn run(setup: Setup, files: &[CaseFile]) -> (CaseVerdict, Vec<String>, String) {
    let log = RefCell::new(Vec::new());
    let make_folder = || {
        log.borrow_mut().push("make".to_string());
        if setup.make_fails {
            return Err(io::Error::other("no space"));
        }
        Ok(PathBuf::from("/case"))
    };
    let write_file = |path: &Path, text: &str| {
        log.borrow_mut().push(format!("write {} {text:?}", path.display()));
        match setup.write_fails {
            Some(name) if path.ends_with(name) => Err(io::Error::other("disk full")),
            _ => Ok(()),
        }
    };
    let remove_folder = |folder: &Path| {
        log.borrow_mut().push(format!("remove {}", folder.display()));
        if setup.remove_fails {
            return Err(io::Error::other("busy"));
        }
        Ok(())
    };
    let run_rules = |rf: &RuleFile, repo: &Path, only: &[RelPath], base: Option<&str>| {
        let ids: Vec<&str> = rf.rules.iter().map(|rule| rule.id.as_str()).collect();
        let ran = format!("run {} {} {ids:?} {} {base:?}", rf.folder.display(), repo.display(), only.len());
        log.borrow_mut().push(ran);
        let run = |outcome| RuleRun::new("tm-zeta".to_string(), Engine::Ruff, outcome, None);
        let run = match setup.not_available {
            Some(cause) => run(RuleOutcome::NotAvailable {
                rule: "tm-zeta".to_string(),
                engine: Engine::Ruff,
                why: "ruff not found on PATH".to_string(),
            })
            .with_cause(cause),
            None => run(RuleOutcome::Findings(setup.findings.clone())),
        };
        CheckedRules { runs: vec![run], engines: vec![] }
    };
    let world = BaselineWorld {
        default_sets: &[],
        rules_of: &|_| unreachable!("rules_of"),
        make_folder: &make_folder,
        write_file: &write_file,
        commit_all: &|_| unreachable!("commit_all"),
        added_since: &|_, _| unreachable!("added_since"),
        remove_folder: &remove_folder,
        run_rules: &run_rules,
    };
    let rule = Rule::new("tm-zeta", Engine::Ruff, "F401");
    let expected = ExpectedFinding { rule_id: "tm-zeta".to_string(), path: "a.py".to_string(), line: 1 };
    let mut err = Vec::new();
    let verdict = run_case(&rule, true, files, &expected, &world, &mut err);
    let steps = log.into_inner();
    (verdict, steps, String::from_utf8(err).unwrap())
}

#[test]
fn run_case_removes_the_folder_whatever_the_case_came_to() {
    let ran = |steps: &[&str]| steps.iter().map(|step| step.to_string()).collect::<Vec<_>>();
    let broken = |why: &str| CaseVerdict::Broken { why: why.to_string() };
    // The writes name `/case` joined part by part, in the platform's separator (`/case/src/b.py` on unix).
    let case = Path::new("/case");
    let write_a = format!("write {} \"import os\\n\"", case.join("a.py").display());
    let write_b = format!("write {} \"import os\\n\"", case.join("src").join("b.py").display());
    let rows = [
        (
            "held",
            Setup { findings: vec![finding(1)], ..OK },
            files(&["a.py", "src/b.py"]),
            CaseVerdict::Holds,
            ran(&[
                "make",
                &write_a,
                &write_b,
                "run /case /case [\"tm-zeta\"] 0 None",
                "remove /case",
            ]),
        ),
        (
            "broken run",
            Setup { findings: vec![finding(2)], ..OK },
            files(&["a.py"]),
            broken("gave tm-zeta at a.py:2; expected tm-zeta at a.py:1"),
            ran(&["make", &write_a, "run /case /case [\"tm-zeta\"] 0 None", "remove /case"]),
        ),
        (
            "not available, with the run's cause",
            Setup { not_available: Some(NotAvailableCause::HelperNotOnPath), ..OK },
            files(&["a.py"]),
            CaseVerdict::NotAvailable {
                why: "ruff not found on PATH".to_string(),
                cause: Some(NotAvailableCause::HelperNotOnPath),
            },
            ran(&["make", &write_a, "run /case /case [\"tm-zeta\"] 0 None", "remove /case"]),
        ),
        (
            "a name above the folder",
            OK,
            files(&["../x.py"]),
            broken("case file \"../x.py\" is not a relative path inside the case folder"),
            ran(&["make", "remove /case"]),
        ),
        (
            "an absolute name, after a file already written",
            OK,
            files(&["a.py", "/abs.py"]),
            broken("case file \"/abs.py\" is not a relative path inside the case folder"),
            ran(&["make", &write_a, "remove /case"]),
        ),
        (
            "a failed write stops the case",
            Setup { write_fails: Some("a.py"), ..OK },
            files(&["a.py", "b.py"]),
            broken("cannot write a.py: disk full"),
            ran(&["make", &write_a, "remove /case"]),
        ),
    ];
    for (name, setup, files, want, want_steps) in rows {
        let (verdict, steps, err) = run(setup, &files);
        assert_eq!(verdict, want, "{name}");
        assert_eq!(steps, want_steps, "{name}");
        assert_eq!(err, "", "{name}");
    }
}

#[test]
fn a_folder_that_cannot_be_made_runs_and_removes_nothing() {
    let (verdict, steps, err) = run(Setup { make_fails: true, ..OK }, &files(&["a.py"]));
    assert_eq!(verdict, CaseVerdict::Broken { why: "cannot make the case folder: no space".to_string() });
    assert_eq!(steps, vec!["make".to_string()]);
    assert_eq!(err, "");
}

#[test]
fn a_folder_that_cannot_be_removed_is_a_line_on_err_and_keeps_the_verdict() {
    let (verdict, steps, err) = run(Setup { remove_fails: true, findings: vec![finding(1)], ..OK }, &files(&["a.py"]));
    assert_eq!(verdict, CaseVerdict::Holds);
    assert_eq!(steps.last().map(String::as_str), Some("remove /case"));
    assert_eq!(err, "cctao: cannot remove the case folder /case: busy\n");
}
