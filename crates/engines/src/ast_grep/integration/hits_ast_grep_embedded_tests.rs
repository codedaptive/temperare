//! ZIT for hits_ast_grep.rs with a rule whose baseline set carries its YAML (`Rule::embedded_yaml`). Integration, not
//! unit: each test makes a real canonical temp folder to run in, and `hits_ast_grep` writes the filled YAML to a real
//! temporary file that exists while the host runs and is gone after. The embedded text is used in place of any file
//! at the check path; with no embedded text the repository file is read. The host is a fake; no engine runs. Each
//! test removes its folder.

use super::super::{hits_ast_grep, AST_GREP};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::RuleFile;
use std::cell::{Cell, RefCell};
use std::fs;
use std::path::PathBuf;

const PLAIN: &str = "id: plain\nlanguage: JavaScript\nrule:\n  regex: '^foo$'\n";
const EMBEDDED: &str = "id: names\nlanguage: JavaScript\nrule:\n  regex: '^(?:{{names}})$'\n";
const EMBEDDED_FILLED: &str = "id: names\nlanguage: JavaScript\nrule:\n  regex: '^(?:foo|a\\.b)$'\n";
const REPOSITORY: &str = "id: repo\nlanguage: Python\nrule:\n  regex: '^(?:{{names}})-repo$'\n";
const REPOSITORY_FILLED: &str = "id: repo\nlanguage: Python\nrule:\n  regex: '^(?:foo|a\\.b)-repo$'\n";
const NAMES: &str = "params = { names = [\"foo\", \"a.b\"] }\n";

/// What the fake run was given: the command, the temp rule it names and that file's text at run time.
struct Seen {
    cmd: Cmd,
    rule: PathBuf,
    text: Option<String>,
}

/// What one call gave: its result, the folder it ran over (removed by then) and what the run saw (None: no run).
struct Ran {
    got: Result<Vec<Hit>, Stop>,
    folder: PathBuf,
    seen: Option<Seen>,
}

/// `hits_ast_grep` for one ast-grep rule checking `names.yml` with `extra` lines and `embedded` as its set's YAML,
/// over a fresh canonical temp folder holding `names.yml` = `repository` (when given); the run answers `answer`.
fn run_embedded(
    test: &str,
    embedded: Option<&'static str>,
    repository: Option<&str>,
    extra: &str,
    answer: impl Fn() -> Result<Output, ProcError>,
) -> Ran {
    let root = std::env::temp_dir().join(format!("engines-zit-{}-hits-ast-grep-embedded-{test}", std::process::id()));
    fs::create_dir(&root).unwrap();
    let folder = fs::canonicalize(root).unwrap();
    if let Some(text) = repository {
        fs::write(folder.join("names.yml"), text).unwrap();
    }
    let text = format!("version = 1\n[[rule]]\nid = \"names\"\nengine = \"ast-grep\"\ncheck = \"names.yml\"\nfix = \"f\"\n{extra}");
    let mut file = RuleFile::parse(&text, &folder).unwrap();
    file.rules[0].embedded_text = embedded;
    let seen = RefCell::new(None);
    let run = |cmd: &Cmd| {
        let at = cmd.args.iter().position(|arg| arg == "--rule").unwrap();
        let rule = PathBuf::from(&cmd.args[at + 1]);
        *seen.borrow_mut() = Some(Seen { cmd: cmd.clone(), text: fs::read_to_string(&rule).ok(), rule });
        answer()
    };
    let on_path = |_: &str| -> bool { panic!("hits_ast_grep looks nothing up on PATH") };
    let host = Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None), base: None };
    let got = hits_ast_grep(&AST_GREP, &file.rules[0], &folder, &host);
    fs::remove_dir_all(&folder).unwrap();
    Ran { got, folder, seen: seen.into_inner() }
}

fn clean() -> Result<Output, ProcError> {
    Ok(Output { code: Some(0), stdout: include_bytes!("../fixtures/clean.jsonl").to_vec(), stderr: Vec::new() })
}

fn matches() -> Result<Output, ProcError> {
    Ok(Output { code: Some(0), stdout: include_bytes!("../fixtures/matches.jsonl").to_vec(), stderr: Vec::new() })
}

fn never_run() -> Result<Output, ProcError> {
    panic!("ast-grep must not run")
}

#[test]
fn embedded_text_reaches_ast_grep_through_a_temporary_file_with_no_repository_file() {
    let ran = run_embedded("plain", Some(PLAIN), None, "", clean);
    assert_eq!(ran.got, Ok(Vec::new()));
    let seen = ran.seen.expect("ast-grep ran");
    assert_eq!(seen.text.as_deref(), Some(PLAIN));
    assert_ne!(seen.rule, ran.folder.join("names.yml"));
    assert_eq!(seen.cmd.cwd, Some(ran.folder));
}

#[test]
fn params_are_filled_into_the_embedded_text() {
    let ran = run_embedded("filled", Some(EMBEDDED), None, NAMES, clean);
    assert_eq!(ran.got, Ok(Vec::new()));
    assert_eq!(ran.seen.expect("ast-grep ran").text.as_deref(), Some(EMBEDDED_FILLED));
}

#[test]
fn the_temporary_file_is_gone_after_findings() {
    let ran = run_embedded("gone-found", Some(EMBEDDED), None, NAMES, matches);
    assert_eq!(ran.got.map(|hits| hits.len()), Ok(4));
    let seen = ran.seen.expect("ast-grep ran");
    assert_eq!(seen.text.as_deref(), Some(EMBEDDED_FILLED));
    assert!(!seen.rule.exists(), "{} is left behind", seen.rule.display());
}

#[test]
fn the_temporary_file_is_gone_after_a_run_error() {
    let answer = || Err(ProcError::NotFound { program: "ast-grep".into() });
    let ran = run_embedded("gone-absent", Some(EMBEDDED), None, NAMES, answer);
    assert_eq!(ran.got, Err(Stop::NotOnPath { program: "ast-grep".to_string() }));
    let seen = ran.seen.expect("ast-grep ran");
    assert_eq!(seen.text.as_deref(), Some(EMBEDDED_FILLED));
    assert!(!seen.rule.exists(), "{} is left behind", seen.rule.display());
}

#[test]
fn the_temporary_file_is_gone_after_rejected_output() {
    let stderr = include_bytes!("../fixtures/broken_rule.stderr");
    let answer = || Ok(Output { code: Some(8), stdout: Vec::new(), stderr: stderr.to_vec() });
    let ran = run_embedded("gone-rejected", Some(EMBEDDED), None, NAMES, answer);
    let want = String::from_utf8_lossy(stderr).into_owned();
    assert_eq!(ran.got, Err(Stop::Failed { code: Some(8), stderr: want }));
    let seen = ran.seen.expect("ast-grep ran");
    assert_eq!(seen.text.as_deref(), Some(EMBEDDED_FILLED));
    assert!(!seen.rule.exists(), "{} is left behind", seen.rule.display());
}

#[test]
fn no_embedded_text_runs_the_repository_file() {
    let ran = run_embedded("repository", None, Some(REPOSITORY), NAMES, clean);
    assert_eq!(ran.got, Ok(Vec::new()));
    assert_eq!(ran.seen.expect("ast-grep ran").text.as_deref(), Some(REPOSITORY_FILLED));
}

#[test]
fn embedded_text_beats_a_repository_file_of_the_same_name() {
    let ran = run_embedded("beats", Some(EMBEDDED), Some(REPOSITORY), NAMES, clean);
    assert_eq!(ran.got, Ok(Vec::new()));
    assert_eq!(ran.seen.expect("ast-grep ran").text.as_deref(), Some(EMBEDDED_FILLED));
}

#[test]
fn an_embedded_param_the_rule_does_not_give_is_failed_and_nothing_runs() {
    let ran = run_embedded("unknown", Some(EMBEDDED), None, "", never_run);
    assert_eq!(ran.got, Err(Stop::Failed { code: None, stderr: "unknown param {{names}} in names.yml".to_string() }));
    assert!(ran.seen.is_none());
}
