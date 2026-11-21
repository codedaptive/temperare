//! ZITs and RITs for `BaselineWorld::real()` and `run_baseline_test`: real case folders, writes, cleanup, and case
//! execution, plus built-in set lookup with effect seams replaced. No engine is run; case outcomes are supplied by
//! test runners. All disk work is in throwaway folders under the system temp folder, removed on drop.

use cli::{run_baseline_test, BaselineArgs, BaselineWorld};
use engines::{CheckedRules, RuleRun};
use rules::{CaseFile, Engine, ExpectedFinding, Finding, Kind, RelPath, Rule, RuleCases, RuleOutcome, Severity};
use std::cell::RefCell;
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(0);

struct TempFolder(PathBuf);

impl TempFolder {
    fn new() -> Self {
        loop {
            let n = NEXT.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!("tm-cli-baseline-test-{}-{n}", std::process::id()));
            match fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(e) if e.kind() == ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("cannot make {}: {e}", path.display()),
            }
        }
    }
}

impl Drop for TempFolder {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn test_rule(failing_path: &str) -> Rule {
    let mut rule = Rule::new("tm-zeta", Engine::Ruff, "F401");
    rule.embedded_cases = Some(RuleCases {
        failing: vec![CaseFile { path: failing_path.into(), text: "import os\n" }],
        passing: vec![CaseFile { path: "pkg/a.py".into(), text: "print(1)\n" }],
        expected: ExpectedFinding { rule_id: "tm-zeta".into(), path: "pkg/a.py".into(), line: 1 },
    });
    rule
}

/// RIT: each real case folder is a distinct empty git repository under the system temp folder.
#[test]
fn make_folder_creates_distinct_empty_git_folders() {
    let world = BaselineWorld::real();
    let first = TempFolder((world.make_folder)().unwrap());
    let second = TempFolder((world.make_folder)().unwrap());
    assert_ne!(first.0, second.0);
    for folder in [&first.0, &second.0] {
        assert_eq!(folder.parent(), Some(std::env::temp_dir().as_path()));
        assert!(folder.is_dir());
        assert!(folder.join(".git").is_dir());
        let entries: Vec<_> = fs::read_dir(folder).unwrap().map(|e| e.unwrap().file_name()).collect();
        assert_eq!(entries, vec![std::ffi::OsString::from(".git")]);
    }
}

/// ZIT: the real disk writer creates parents and writes the exact case text.
#[test]
fn write_file_creates_parents_and_writes_text() {
    let world = BaselineWorld::real();
    let folder = TempFolder::new();
    let file = folder.0.join("src/deep/a.py");
    (world.write_file)(&file, "import os\n").unwrap();
    assert_eq!(fs::read_to_string(file).unwrap(), "import os\n");
}

/// ZIT: the real disk remover removes a populated folder recursively.
#[test]
fn remove_folder_removes_everything() {
    let world = BaselineWorld::real();
    let folder = TempFolder::new();
    fs::create_dir(folder.0.join("nested")).unwrap();
    fs::write(folder.0.join("nested/a.py"), "text").unwrap();
    (world.remove_folder)(&folder.0).unwrap();
    assert!(!folder.0.exists());
}

/// RIT: the real rules lookup resolves the standard set and rejects an unknown set.
#[test]
fn rules_of_resolves_and_rejects_sets() {
    let world = BaselineWorld::real();
    assert!((world.rules_of)("standard@1").is_ok());
    assert_eq!((world.rules_of)("nosuch@1").unwrap_err()[0].what, "unknown baseline set \"nosuch\"");
}

fn run_with_real_sets(set: Option<&str>) -> (u8, String, String, Vec<String>) {
    let real = BaselineWorld::real();
    let read = RefCell::new(Vec::new());
    let rules_of = |set: &str| {
        read.borrow_mut().push(set.to_string());
        (real.rules_of)(set)
    };
    let run_rules = |rf: &rules::RuleFile, _: &std::path::Path, _: &[RelPath], _: Option<&str>| {
        let rule = &rf.rules[0];
        let outcome = RuleOutcome::NotAvailable {
            rule: rule.id.clone(), engine: rule.engine, why: "engine unavailable".to_string(),
        };
        CheckedRules { runs: vec![RuleRun::new(rule.id.clone(), rule.engine, outcome, None)], engines: vec![] }
    };
    let world = BaselineWorld {
        rules_of: &rules_of,
        make_folder: &|| Ok(PathBuf::from("/nonexistent-tm-baseline-case")),
        write_file: &|_, _| Ok(()),
        commit_all: &|_| Ok("base".to_string()),
        added_since: &|_, _| Ok(Default::default()),
        remove_folder: &|_| Ok(()),
        run_rules: &run_rules,
        ..real
    };
    let args = BaselineArgs { set: set.map(str::to_string), json: false };
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let exit = run_baseline_test(&args, &world, &mut out, &mut err);
    (exit, String::from_utf8(out).unwrap(), String::from_utf8(err).unwrap(), read.into_inner())
}

/// RIT: the house set prints every real rule in order, with the total and no held or broken rules.
#[test]
fn real_sets_house_runs() {
    let (exit, out, err, _) = run_with_real_sets(Some("house@1"));
    assert_eq!(exit, 3, "stderr: {err}");
    assert_eq!(err, "");
    let expected: Vec<_> = rules::baseline_rules("house@1").unwrap().into_iter().map(|rule| rule.id).collect();
    let printed: Vec<_> = out.lines().filter(|line| !line.starts_with(' ') && line.contains(" ("))
        .map(|line| {
            let (set, rest) = line.split_once(' ').unwrap();
            assert_eq!(set, "house@1");
            rest.split_once(" (").unwrap().0
        }).collect();
    assert_eq!(printed, expected);
    let counts = out.lines().find(|line| line.contains(" rules: ")).unwrap();
    assert!(counts.starts_with(&format!("{} rules:", expected.len())), "{counts}");
    assert!(counts.contains(": 0 hold, 0 broken,"), "{counts}");
}

/// RIT: a required-param house rule runs both real cases with their own params.
#[test]
fn real_house_required_params_run_both_cases() {
    let target = "tm-house-unit-test-fakes-own-kit-python";
    let rule = rules::baseline_rules("house@1").unwrap().into_iter().find(|rule| rule.id == target).unwrap();
    let expected = &rule.cases().unwrap().expected;
    let line = u32::try_from(expected.line).unwrap();
    let seen = RefCell::new(Vec::new());
    let run_rules = |rf: &rules::RuleFile, _: &std::path::Path, _: &[RelPath], _: Option<&str>| {
        let rule = &rf.rules[0];
        let outcome = if rule.id != target {
            RuleOutcome::NotAvailable {
                rule: rule.id.clone(), engine: rule.engine, why: "engine unavailable".to_string(),
            }
        } else {
            let mut seen = seen.borrow_mut();
            seen.push(rule.params.clone());
            let findings = if seen.len() == 1
                && rule.params.get("own_package") == Some(&toml::Value::Array(vec!["ledger".into()]))
            {
                vec![Finding::new(
                    &expected.rule_id, (rule.engine, rule.kind, rule.severity), RelPath::new(&expected.path),
                    line, None, "fake finding", "fix it",
                )]
            } else {
                Vec::new()
            };
            RuleOutcome::Findings(findings)
        };
        CheckedRules { runs: vec![RuleRun::new(rule.id.clone(), rule.engine, outcome, None)], engines: vec![] }
    };
    let added_since = |_: &std::path::Path, _: &str| {
        if seen.borrow().len() == 1 {
            Ok([(PathBuf::from(&expected.path), vec![line..line + 1])].into())
        } else {
            Ok(Default::default())
        }
    };
    let world = BaselineWorld {
        make_folder: &|| Ok(PathBuf::from("/nonexistent-tm-baseline-case")),
        write_file: &|_, _| Ok(()),
        commit_all: &|_| Ok("base".to_string()),
        added_since: &added_since,
        remove_folder: &|_| Ok(()),
        run_rules: &run_rules,
        ..BaselineWorld::real()
    };
    let args = BaselineArgs { set: Some("house@1".into()), json: false };
    let mut out = Vec::new();
    run_baseline_test(&args, &world, &mut out, &mut Vec::new());
    let out = String::from_utf8(out).unwrap();
    assert!(out.lines().any(|line| line == "house@1 tm-house-unit-test-fakes-own-kit-python (ast-grep): holds"), "{out}");
    let seen = seen.borrow();
    assert_eq!(seen.len(), 2);
    for params in seen.iter() {
        assert_eq!(params.get("own_package"), Some(&toml::Value::Array(vec!["ledger".into()])));
    }
}

/// RIT: without a named set, the real world reads exactly the sets `rules::baseline_sets` marks ready, in its order.
/// Which sets are ready is rules' data, so the expected list comes from there, and it must not be empty.
#[test]
fn real_sets_default_reads_ready_only() {
    let ready: Vec<String> =
        rules::baseline_sets().into_iter().filter(|(_, ready)| *ready).map(|(set, _)| set).collect();
    assert!(!ready.is_empty(), "rules marks no baseline set ready");
    let (_, _, _, read) = run_with_real_sets(None);
    assert_eq!(read, ready);
}

/// RIT: real case files exist when the runner reads them, then both case folders are removed.
#[test]
fn run_baseline_test_writes_cases_and_removes_folders() {
    let rule = test_rule("pkg/a.py");
    let rules_of = |set: &str| {
        assert_eq!(set, "test@1");
        Ok(vec![rule.clone()])
    };
    let seen = RefCell::new(Vec::new());
    let run_rules = |_: &rules::RuleFile, folder: &std::path::Path, _: &[RelPath], _: Option<&str>| {
        let folder = TempFolder(folder.to_path_buf());
        let text = fs::read_to_string(folder.0.join("pkg/a.py")).unwrap();
        let findings = if text == "import os\n" {
            vec![Finding::new(
                "tm-zeta", (Engine::Ruff, Kind::Quality, Severity::Error), RelPath::new("pkg/a.py"),
                1_u32, None, "unused import", "remove it",
            )]
        } else {
            Vec::new()
        };
        seen.borrow_mut().push((folder, text));
        CheckedRules {
            runs: vec![RuleRun::new("tm-zeta".into(), Engine::Ruff, RuleOutcome::Findings(findings), None)],
            engines: Vec::new(),
        }
    };
    let world = BaselineWorld { rules_of: &rules_of, run_rules: &run_rules, ..BaselineWorld::real() };
    let args = BaselineArgs { set: Some("test@1".into()), json: false };
    assert_eq!(run_baseline_test(&args, &world, &mut Vec::new(), &mut Vec::new()), 0);
    let seen = seen.borrow();
    assert_eq!(seen.len(), 2);
    assert_eq!(seen[0].1, "import os\n");
    assert_eq!(seen[1].1, "print(1)\n");
    assert_ne!(seen[0].0.0, seen[1].0.0);
    for (folder, _) in seen.iter() {
        assert!(!folder.0.exists(), "case folder remains: {}", folder.0.display());
    }
}

/// RIT: an escaping case is rejected, leaving neither its case folder nor a file beside it.
#[test]
fn run_baseline_test_rejects_escape_and_removes_folder() {
    let parent = TempFolder::new();
    let real = BaselineWorld::real();
    let made = RefCell::new(Vec::new());
    let make_folder = || {
        let original = TempFolder((real.make_folder)()?);
        let folder = parent.0.join(original.0.file_name().unwrap());
        fs::rename(&original.0, &folder)?;
        made.borrow_mut().push(folder.clone());
        Ok(folder)
    };
    let rule = test_rule("../escape.py");
    let rules_of = |set: &str| {
        assert_eq!(set, "test@1");
        Ok(vec![rule.clone()])
    };
    let run_rules = |_: &rules::RuleFile, _: &std::path::Path, _: &[RelPath], _: Option<&str>| CheckedRules {
        runs: vec![RuleRun::new("tm-zeta".into(), Engine::Ruff, RuleOutcome::Findings(Vec::new()), None)],
        engines: Vec::new(),
    };
    let world = BaselineWorld { make_folder: &make_folder, rules_of: &rules_of, run_rules: &run_rules, ..real };
    let args = BaselineArgs { set: Some("test@1".into()), json: false };
    assert_eq!(run_baseline_test(&args, &world, &mut Vec::new(), &mut Vec::new()), 1);
    let made = made.borrow();
    assert_eq!(made.len(), 2);
    for folder in made.iter() {
        assert!(!folder.exists(), "case folder remains: {}", folder.display());
    }
    assert!(!parent.0.join("escape.py").exists());
}

/// RIT: a change-only rule keeps the added-line finding and discards the unchanged-line finding.
#[test]
fn run_baseline_test_filters_findings_to_lines_added_since_base() {
    let mut rule = Rule::new("tm-change-adds-sleep", Engine::AstGrep, "sleep.yml");
    rule.only_with_base = true;
    rule.embedded_cases = Some(RuleCases {
        failing: vec![CaseFile { path: "a.py".into(), text: "x = 1\nsleep(1)\n" }],
        passing: vec![CaseFile { path: "a.py".into(), text: "x = 1\n" }],
        expected: ExpectedFinding { rule_id: "tm-change-adds-sleep".into(), path: "a.py".into(), line: 2 },
    });
    let rules_of = |set: &str| {
        assert_eq!(set, "test@1");
        Ok(vec![rule.clone()])
    };
    let real = BaselineWorld::real();
    let folders = RefCell::new(Vec::new());
    let make_folder = || {
        let folder = (real.make_folder)()?;
        folders.borrow_mut().push(TempFolder(folder.clone()));
        Ok(folder)
    };
    let run_rules = |_: &rules::RuleFile, folder: &std::path::Path, _: &[RelPath], _: Option<&str>| {
        let text = fs::read_to_string(folder.join("a.py")).unwrap();
        let findings = if text.contains("sleep") {
            [1_u32, 2].into_iter().map(|line| Finding::new(
                "tm-change-adds-sleep", (Engine::AstGrep, Kind::Quality, Severity::Error),
                RelPath::new("a.py"), line, None, "sleep", "remove it",
            )).collect()
        } else {
            Vec::new()
        };
        CheckedRules {
            runs: vec![RuleRun::new("tm-change-adds-sleep".into(), Engine::AstGrep, RuleOutcome::Findings(findings), None)],
            engines: Vec::new(),
        }
    };
    let world = BaselineWorld { make_folder: &make_folder, rules_of: &rules_of, run_rules: &run_rules, ..real };
    let args = BaselineArgs { set: Some("test@1".into()), json: false };
    let mut out = Vec::new();
    let exit = run_baseline_test(&args, &world, &mut out, &mut Vec::new());
    let folders = folders.borrow();
    assert_eq!(folders.len(), 2);
    for folder in folders.iter() {
        assert!(!folder.0.exists(), "case folder remains: {}", folder.0.display());
    }
    assert_eq!(exit, 0, "output: {}", String::from_utf8_lossy(&out));
    assert!(String::from_utf8(out).unwrap().lines().any(|line| {
        line == "test@1 tm-change-adds-sleep (ast-grep): holds"
    }));
}
