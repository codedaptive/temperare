//! ZUT for run_change_case.rs, with recording closures: no disk, no git, no engine. Every step the case takes on its
//! world is logged, so each row shows the order of the writes, the base commit, the run and the folder's removal.

use super::run_change_case::run_change_case;
use super::BaselineWorld;
use crate::output::CaseVerdict;
use engines::{CheckedRules, RuleRun};
use rules::{
    CaseFile, Engine, ExpectedFinding, Finding, Kind, RelPath, Rule, RuleCases, RuleFile, RuleOutcome, Severity,
};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf};

/// How the world answers: whether make_folder, commit_all and added_since fail, and the outcome of the rule's run.
/// Line 2 of a.py is the one line added since the base.
struct Setup {
    make_fails: bool,
    commit_fails: bool,
    added_fails: bool,
    outcome: RuleOutcome,
}

fn found(lines: &[u32]) -> RuleOutcome {
    let place = (Engine::AstGrep, Kind::Quality, Severity::Error);
    RuleOutcome::Findings(
        lines.iter().map(|line| Finding::new("tm-sleep", place, RelPath::new("a.py"), *line, None, "w", "f")).collect(),
    )
}

fn ok(outcome: RuleOutcome) -> Setup {
    Setup { make_fails: false, commit_fails: false, added_fails: false, outcome }
}

/// Runs the failing (`failing`) or passing case of tm-sleep (passing a.py `x = 1`, failing a.py `x = 1` then
/// `sleep(1)`, expected at a.py:2) in the world `setup` describes; (verdict, the steps taken, what was written on err).
fn run(setup: Setup, failing: bool) -> (CaseVerdict, Vec<String>, String) {
    let log = RefCell::new(Vec::new());
    let step = |text: String| log.borrow_mut().push(text);
    let make_folder = || {
        step("make".to_string());
        if setup.make_fails {
            return Err(io::Error::other("no space"));
        }
        Ok(PathBuf::from("/case"))
    };
    let write_file = |path: &Path, text: &str| {
        step(format!("write {} {text:?}", path.display()));
        Ok(())
    };
    let commit_all = |folder: &Path| {
        step(format!("commit {}", folder.display()));
        if setup.commit_fails {
            return Err(io::Error::other("no git"));
        }
        Ok("b45e".to_string())
    };
    let added_since = |folder: &Path, base: &str| {
        step(format!("added {} {base}", folder.display()));
        if setup.added_fails {
            return Err(io::Error::other("bad diff"));
        }
        Ok(BTreeMap::from([(PathBuf::from("a.py"), vec![2..3])]))
    };
    let run_rules = |rf: &RuleFile, repo: &Path, only: &[RelPath]| {
        let ids: Vec<&str> = rf.rules.iter().map(|rule| rule.id.as_str()).collect();
        step(format!("run {} {} {ids:?} {}", rf.folder.display(), repo.display(), only.len()));
        CheckedRules {
            runs: vec![RuleRun::new("tm-sleep".to_string(), Engine::AstGrep, setup.outcome.clone(), None)],
            engines: vec![],
        }
    };
    let world = BaselineWorld {
        default_sets: &[],
        rules_of: &|_| unreachable!("rules_of"),
        make_folder: &make_folder,
        write_file: &write_file,
        commit_all: &commit_all,
        added_since: &added_since,
        remove_folder: &|folder| {
            step(format!("remove {}", folder.display()));
            Ok(())
        },
        run_rules: &run_rules,
    };
    let mut rule = Rule::new("tm-sleep", Engine::AstGrep, "sleep.yml");
    rule.only_with_base = true;
    let cases = RuleCases {
        failing: vec![CaseFile { path: "a.py".to_string(), text: "x = 1\nsleep(1)\n" }],
        passing: vec![CaseFile { path: "a.py".to_string(), text: "x = 1\n" }],
        expected: ExpectedFinding { rule_id: "tm-sleep".to_string(), path: "a.py".to_string(), line: 2 },
    };
    let mut err = Vec::new();
    let verdict = run_change_case(&rule, failing, &cases, &world, &mut err);
    let steps = log.borrow().clone();
    (verdict, steps, String::from_utf8(err).unwrap())
}

#[test]
fn run_change_case_rows() {
    let ran = |steps: &[&str]| steps.iter().map(|step| step.to_string()).collect::<Vec<_>>();
    let broken = |why: &str| CaseVerdict::Broken { why: why.to_string() };
    let base = "write /case/a.py \"x = 1\\n\"";
    let change = "write /case/a.py \"x = 1\\nsleep(1)\\n\"";
    let ruled = "run /case /case [\"tm-sleep\"] 0";
    let rows = [
        (
            "failing: the base holds the passing files, the unchanged line's finding is dropped",
            ok(found(&[1, 2])),
            true,
            CaseVerdict::Holds,
            ran(&["make", base, "commit /case", change, ruled, "added /case b45e", "remove /case"]),
        ),
        (
            "failing: a finding on the unchanged line alone is no finding",
            ok(found(&[1])),
            true,
            broken("gave no finding; expected tm-sleep at a.py:2"),
            ran(&["make", base, "commit /case", change, ruled, "added /case b45e", "remove /case"]),
        ),
        (
            "passing: the base is the empty folder",
            ok(found(&[])),
            false,
            CaseVerdict::Holds,
            ran(&["make", "commit /case", base, ruled, "added /case b45e", "remove /case"]),
        ),
        (
            "passing: a finding on an added line is kept",
            ok(found(&[2])),
            false,
            broken("gave tm-sleep at a.py:2; expected no finding"),
            ran(&["make", "commit /case", base, ruled, "added /case b45e", "remove /case"]),
        ),
        (
            "a base that cannot be committed runs nothing",
            Setup { commit_fails: true, ..ok(found(&[])) },
            true,
            broken("cannot commit the case's base: no git"),
            ran(&["make", base, "commit /case", "remove /case"]),
        ),
        (
            "added lines that cannot be read",
            Setup { added_fails: true, ..ok(found(&[2])) },
            true,
            broken("cannot read the lines the case adds: bad diff"),
            ran(&["make", base, "commit /case", change, ruled, "added /case b45e", "remove /case"]),
        ),
        (
            "an outcome with no findings is judged as it is",
            ok(RuleOutcome::NotAvailable {
                rule: "tm-sleep".to_string(),
                engine: Engine::AstGrep,
                why: "ast-grep not found on PATH".to_string(),
            }),
            true,
            CaseVerdict::NotAvailable { why: "ast-grep not found on PATH".to_string() },
            ran(&["make", base, "commit /case", change, ruled, "remove /case"]),
        ),
        (
            "a folder that cannot be made runs and removes nothing",
            Setup { make_fails: true, ..ok(found(&[])) },
            true,
            broken("cannot make the case folder: no space"),
            ran(&["make"]),
        ),
    ];
    for (name, setup, failing, want, want_steps) in rows {
        let (verdict, steps, err) = run(setup, failing);
        assert_eq!(verdict, want, "{name}");
        assert_eq!(steps, want_steps, "{name}");
        assert_eq!(err, "", "{name}");
    }
}
