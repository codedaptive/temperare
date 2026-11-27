//! Unit tests for run_git_with.rs with a fake reader of the change and a fake host: the hit over the ratio, none under it or
//! under the floor, a test-only change, no base, each param the flow cannot read, a reader's stop, and the rule's scope
//! given to the reader. No git runs; every expected text is written out from the contract.

use super::{run_git_with, GIT};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::{RelPath, Rule, RuleFile};
use std::cell::{Cell, RefCell};
use std::ops::Range;
use std::path::{Path, PathBuf};

type Change = Vec<(RelPath, Vec<Range<u32>>)>;

const FOLDER: &str = "/no/such/engines-zut-git-folder";
const PARAMS: &str = "max_ratio = 1.5, floor = 40, test_paths = [\"tests/**\"]";

fn rule(check: &str, params: &str) -> Rule {
    let text = format!(
        "version = 1\n[[rule]]\nid = \"r\"\nengine = \"git\"\ncheck = \"{check}\"\nfix = \"f\"\npaths = [\"src/**\", \"tests/**\"]\nparams = {{ {params} }}\n"
    );
    RuleFile::parse(&text, Path::new(FOLDER)).unwrap().rules.remove(0)
}

fn change(given: &[(&str, &[Range<u32>])]) -> Result<Change, Stop> {
    Ok(given.iter().map(|(path, ranges)| (RelPath::new(path), ranges.to_vec())).collect())
}

/// `run_git_with` on `rule` with the reader giving `given` and the host carrying `base`; also each (folder, base)
/// the reader was asked for, the programs the host ran and what `raw_chars` holds after.
fn run(rule: &Rule, base: Option<&str>, given: Result<Change, Stop>) -> (Result<Vec<Hit>, Stop>, Vec<(PathBuf, String)>, usize, Option<usize>) {
    let asked = RefCell::new(Vec::new());
    let read = |folder: &Path, base: &str, _: &dyn Fn(&RelPath) -> bool| {
        asked.borrow_mut().push((folder.to_path_buf(), base.to_string()));
        given.clone()
    };
    let ran = Cell::new(0);
    let host_run = |_: &Cmd| -> Result<Output, ProcError> {
        ran.set(ran.get() + 1);
        Ok(Output { code: Some(0), stdout: Vec::new(), stderr: Vec::new() })
    };
    let raw_chars = Cell::new(None);
    let host = Host { run: &host_run, on_path: &|_| true, raw_chars: &raw_chars, base };
    let got = run_git_with(&read, &GIT, rule, Path::new(FOLDER), &host);
    (got, asked.into_inner(), ran.get(), raw_chars.get())
}

fn hit(path: &str, line: u32, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line: Some(line), col: None, what: what.to_string() }
}

fn failed(stderr: &str) -> Result<Vec<Hit>, Stop> {
    Err(Stop::Failed { code: None, stderr: stderr.to_string() })
}

#[test]
fn a_change_is_judged_against_the_ratio_and_the_floor() {
    let over = change(&[("src/a.rs", &[1..11]), ("tests/a_tests.rs", &[2..32]), ("tests/new_tests.rs", &[1..51])]);
    let over_what = "this change adds 80 lines of test for 10 lines of code (8.0 to 1; the rule's limit is 1.5 past 40 lines)";
    let test_only = "this change adds 45 lines of test for 0 lines of code (test only; the rule's limit is 2 past 40 lines)";
    let cases = [
        ("over the ratio", PARAMS, over, vec![hit("tests/new_tests.rs", 1, over_what)]),
        ("under the ratio", PARAMS, change(&[("src/a.rs", &[1..31]), ("tests/t.rs", &[5..45])]), vec![]),
        ("under the floor", PARAMS, change(&[("src/a.rs", &[1..3]), ("tests/t.rs", &[1..21])]), vec![]),
        (
            "no code",
            "max_ratio = 2, floor = 40, test_paths = [\"tests/**\"]",
            change(&[("tests/t.rs", &[3..48])]),
            vec![hit("tests/t.rs", 3, test_only)],
        ),
        ("no change", PARAMS, change(&[]), vec![]),
    ];
    for (name, params, given, want) in cases {
        let (got, asked, ran, raw_chars) = run(&rule("test_ratio", params), Some("abc123"), given);
        assert_eq!(got, Ok(want), "{name}");
        assert_eq!(asked, vec![(PathBuf::from(FOLDER), "abc123".to_string())], "{name}");
        assert_eq!((ran, raw_chars), (0, None), "{name}");
    }
}

#[test]
fn no_base_is_not_available_and_the_change_is_not_read() {
    let (got, asked, ran, raw_chars) = run(&rule("test_ratio", PARAMS), None, change(&[("tests/t.rs", &[1..90])]));
    assert_eq!(got, Err(Stop::NotAvailable { why: "reports only what a change adds; run with --base".to_string() }));
    assert!(asked.is_empty(), "{asked:?}");
    assert_eq!((ran, raw_chars), (0, None));
}

#[test]
fn a_param_the_flow_cannot_read_fails_before_the_change_is_read() {
    let cases = [
        ("test_ratio", "max_ratio = 0, floor = 40, test_paths = [\"t/**\"]", "git param max_ratio: must be a number from 0.1 to 100"),
        ("test_ratio", "max_ratio = 1.5, floor = 100001, test_paths = [\"t/**\"]", "git param floor: must be a whole number from 0 to 100000"),
        ("test_ratio", "max_ratio = 1.5, floor = 40, test_paths = []", "git param test_paths: must be a list of glob strings"),
        ("test_ratio", "max_ratio = 1.5, test_paths = [\"t/**\"]", "git param floor: a git rule needs it"),
    ];
    for (check, params, want) in cases {
        let (got, asked, ran, _) = run(&rule(check, params), Some("abc123"), change(&[("tests/t.rs", &[1..90])]));
        assert_eq!(got, failed(want), "{check} {params}");
        assert!(asked.is_empty(), "{check} {params}: {asked:?}");
        assert_eq!(ran, 0, "{check} {params}");
    }
}

#[test]
fn the_readers_stop_is_the_rules() {
    let stop = Stop::Failed { code: None, stderr: "abc123: not a revision in this repository".to_string() };
    let (got, _, _, raw_chars) = run(&rule("test_ratio", PARAMS), Some("abc123"), Err(stop));
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: "abc123: not a revision in this repository".to_string() }));
    assert_eq!(raw_chars, None);
}

#[test]
fn the_reader_keeps_only_the_paths_the_rule_covers() {
    // The rule's paths are src/** and tests/**, so of these the reader may keep only the first and the third.
    let listed = ["src/a.rs", "docs/a.md", "tests/t.rs", "a.rs"];
    let kept: RefCell<Vec<&str>> = RefCell::new(Vec::new());
    let read = |_: &Path, _: &str, keep: &dyn Fn(&RelPath) -> bool| {
        kept.borrow_mut().extend(listed.iter().filter(|path| keep(&RelPath::new(path))));
        change(&[])
    };
    let host = Host { run: &|_| unreachable!(), on_path: &|_| true, raw_chars: &Cell::new(None), base: Some("abc123") };
    let got = run_git_with(&read, &GIT, &rule("test_ratio", PARAMS), Path::new(FOLDER), &host);
    assert_eq!(got, Ok(vec![]));
    assert_eq!(kept.into_inner(), vec!["src/a.rs", "tests/t.rs"]);
}
