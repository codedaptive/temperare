//! Zone integration tests for hits_ast_grep.rs. These are integration, not unit: each test makes a real canonical temp folder that
//! `hits_ast_grep` reads the rule YAML from through the real `read_rule_yaml`, fills it from the rule's params into a
//! real temporary copy that exists while the host runs and is gone after, on every path; param and read errors; a
//! check that names a file outside the folder is refused and nothing runs; garbage output and an absent ast-grep.
//! The host is a fake; no engine runs. Matches come from the captured fixtures (see parse_ast_grep_tests.rs), whose
//! paths are relative to where ast-grep ran. Each test removes its folder.

use super::super::hits_ast_grep;
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::{RelPath, RuleFile};
use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};

const YAML: &str = "id: names\nlanguage: JavaScript\nrule:\n  regex: '^(?:{{names}})$'\n";
const FILLED: &str = "id: names\nlanguage: JavaScript\nrule:\n  regex: '^(?:foo|a\\.b)$'\n";
const NAMES: &str = "params = { names = [\"foo\", \"a.b\"] }\n";

/// What the fake run was given: the command, the temp rule it names and that file's text at run time (None when it
/// could not be read then).
struct Seen {
    cmd: Cmd,
    rule: PathBuf,
    text: Option<String>,
}

/// A rule file text with one ast-grep rule checking `check`, plus `extra` lines.
fn rule_text(check: &str, extra: &str) -> String {
    format!("version = 1\n[[rule]]\nid = \"names\"\nengine = \"ast-grep\"\ncheck = \"{check}\"\nfix = \"f\"\n{extra}")
}

/// `hits_ast_grep` for the one rule in `text`, over a fresh canonical temp folder holding `names.yml` = `yaml` (when
/// given), removed again before returning; the host's run answers `answer`. Also the folder and what the run saw.
fn run_rule(
    test: &str,
    yaml: Option<&str>,
    text: &str,
    answer: impl Fn() -> Result<Output, ProcError>,
) -> (Result<Vec<Hit>, Stop>, PathBuf, Option<Seen>) {
    let root = std::env::temp_dir().join(format!("engines-zit-{}-hits-ast-grep-{test}", std::process::id()));
    fs::create_dir(&root).unwrap();
    let folder = fs::canonicalize(root).unwrap();
    if let Some(yaml) = yaml {
        fs::write(folder.join("names.yml"), yaml).unwrap();
    }
    let file = RuleFile::parse(text, &folder).unwrap();
    let seen = RefCell::new(None);
    let run = |cmd: &Cmd| {
        let at = cmd.args.iter().position(|arg| arg == "--rule").unwrap();
        let rule = PathBuf::from(&cmd.args[at + 1]);
        *seen.borrow_mut() = Some(Seen { cmd: cmd.clone(), text: fs::read_to_string(&rule).ok(), rule });
        answer()
    };
    let on_path = |_: &str| -> bool { panic!("hits_ast_grep looks nothing up on PATH") };
    let got = hits_ast_grep(&file.rules[0], &folder, &Host { run: &run, on_path: &on_path });
    fs::remove_dir_all(&folder).unwrap();
    (got, folder, seen.into_inner())
}

fn matches() -> Result<Output, ProcError> {
    Ok(Output { code: Some(0), stdout: include_bytes!("../fixtures/matches.jsonl").to_vec(), stderr: Vec::new() })
}

fn never_run() -> Result<Output, ProcError> {
    panic!("ast-grep must not run")
}

fn hit(path: &str, line: u32, col: u32, what: &str) -> Hit {
    Hit { path: RelPath::new(path), line, col: Some(col), what: what.to_string() }
}

#[test]
fn findings_come_through_from_a_filled_copy_that_is_gone_after() {
    let (got, folder, seen) = run_rule("found", Some(YAML), &rule_text("names.yml", NAMES), matches);
    let seen = seen.unwrap();
    assert_eq!(
        got,
        Ok(vec![
            hit("a.js", 1, 13, "matched js name"),
            hit("a.js", 2, 13, "matched js name"),
            hit("a.js", 3, 9, "matched js name"),
            hit("b.py", 2, 1, "py-names"),
        ])
    );
    assert_eq!(seen.text.as_deref(), Some(FILLED));
    assert!(!seen.rule.exists(), "{} is left behind", seen.rule.display());
    assert_eq!(seen.cmd.cwd, Some(folder));
}

#[test]
fn garbage_output_is_failed_with_the_exit_code_and_the_copy_is_gone() {
    let stderr = include_bytes!("../fixtures/broken_rule.stderr");
    let answer = || Ok(Output { code: Some(8), stdout: Vec::new(), stderr: stderr.to_vec() });
    let (got, _, seen) = run_rule("garbage", Some(YAML), &rule_text("names.yml", NAMES), answer);
    let want = String::from_utf8_lossy(stderr).into_owned();
    assert_eq!(got, Err(Stop::Failed { code: Some(8), stderr: want }));
    assert!(!seen.unwrap().rule.exists());
}

#[test]
fn ast_grep_not_found_is_not_available_and_the_copy_is_gone() {
    let answer = || Err(ProcError::NotFound { program: "ast-grep".into() });
    let (got, _, seen) = run_rule("absent", Some(YAML), &rule_text("names.yml", NAMES), answer);
    assert_eq!(got, Err(Stop::NotAvailable { why: "ast-grep not found on PATH".to_string() }));
    let seen = seen.unwrap();
    assert_eq!(seen.text.as_deref(), Some(FILLED));
    assert!(!seen.rule.exists());
}

#[test]
fn a_param_the_rule_does_not_give_is_failed_and_nothing_runs() {
    let (got, _, _) = run_rule("unknown", Some(YAML), &rule_text("names.yml", ""), never_run);
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: "unknown param {{names}} in names.yml".to_string() }));
}

#[test]
fn a_param_that_cannot_fill_is_failed_and_nothing_runs() {
    let (got, _, _) = run_rule("bad", Some(YAML), &rule_text("names.yml", "params = { names = 3 }\n"), never_run);
    let want = "param names in names.yml: not a string or a list of strings".to_string();
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: want }));
}

#[test]
fn a_rule_yaml_that_cannot_be_read_is_failed_and_nothing_runs() {
    let (got, _, _) = run_rule("unreadable", None, &rule_text("names.yml", NAMES), never_run);
    match got {
        Err(Stop::Failed { code: None, stderr }) => {
            assert!(stderr.starts_with("cannot read ast-grep rule names.yml: "), "{stderr}");
        }
        other => panic!("not Failed with no code: {other:?}"),
    }
}

fn outside_case(test: &str, make_check: impl FnOnce(&Path) -> String) -> (Result<Vec<Hit>, Stop>, String) {
    let root = std::env::temp_dir().join(format!("engines-zit-{}-ast-grep-reject-{test}", std::process::id()));
    fs::create_dir(&root).unwrap();
    let root = fs::canonicalize(root).unwrap();
    let folder = root.join("rules");
    fs::create_dir(&folder).unwrap();
    fs::write(root.join("outside.yml"), YAML).unwrap();
    fs::write(folder.join("inside.yml"), YAML).unwrap();
    let check = make_check(&folder);
    let file = RuleFile::parse(&rule_text(&check, ""), &folder).unwrap();
    let run = |_: &Cmd| never_run();
    let on_path = |_: &str| -> bool { panic!("hits_ast_grep looks nothing up on PATH") };
    let got = hits_ast_grep(&file.rules[0], &folder, &Host { run: &run, on_path: &on_path });
    fs::remove_dir_all(&root).unwrap();
    (got, check)
}

#[test]
fn refuses_absolute_check_outside_folder() {
    let (got, check) = outside_case("absolute-outside", |folder| folder.parent().unwrap().join("outside.yml").to_str().unwrap().to_string());
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: format!("ast-grep rule {check} is an absolute path: a check names a file inside the rule file's folder") }));
}

#[test]
fn refuses_absolute_check_inside_folder() {
    let (got, check) = outside_case("absolute-inside", |folder| folder.join("inside.yml").to_str().unwrap().to_string());
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: format!("ast-grep rule {check} is an absolute path: a check names a file inside the rule file's folder") }));
}

#[test]
fn refuses_parent_check_outside_folder() {
    let (got, _) = outside_case("parent", |_| "../outside.yml".to_string());
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: "ast-grep rule ../outside.yml is outside the rule file's folder".to_string() }));
}

#[test]
fn refuses_symlink_check_outside_folder() {
    let (got, _) = outside_case("symlink", |folder| {
        std::os::unix::fs::symlink(folder.parent().unwrap().join("outside.yml"), folder.join("link.yml")).unwrap();
        "link.yml".to_string()
    });
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: "ast-grep rule link.yml is outside the rule file's folder".to_string() }));
}
