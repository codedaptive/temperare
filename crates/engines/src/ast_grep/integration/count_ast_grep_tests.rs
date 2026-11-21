//! ZIT for count_ast_grep.rs, through `hits_ast_grep` with a rule in count mode. Integration, not unit: each test makes
//! a real canonical temp folder to run in, and the filled YAML is written to a real temporary file that exists while
//! the host runs and is gone after. The host is a fake; no engine runs, and no test here reaches git: every run either
//! stops before ast-grep or gives output that is not an answer. Reporting on a real change is tests/ast_grep_count.rs.
//! Each test removes its folder.

use super::super::{hits_ast_grep, AST_GREP};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::{RuleFile, ONLY_WITH_BASE_WHY};
use std::cell::{Cell, RefCell};
use std::fs;
use std::path::PathBuf;

const PLAIN: &str = "id: root\nlanguage: Rust\nrule:\n  kind: source_file\n";
const NAMES: &str = "id: names\nlanguage: Rust\nrule:\n  regex: '^(?:{{names}})$'\n";
const NAMES_FILLED: &str = "id: names\nlanguage: Rust\nrule:\n  regex: '^(?:foo|a\\.b)$'\n";

/// What the fake run saw: the temp rule it was named and that file's text at run time.
struct Seen {
    rule: PathBuf,
    text: Option<String>,
}

/// `hits_ast_grep` for one ast-grep rule checking `names.yml` with `params` and `embedded` as its set's YAML, over a
/// fresh canonical temp folder, with `base` as the host's base; the run answers `answer`. Also what the run saw.
fn run_count(
    test: &str,
    embedded: &'static str,
    params: &str,
    base: Option<&str>,
    answer: impl Fn() -> Result<Output, ProcError>,
) -> (Result<Vec<Hit>, Stop>, Option<Seen>) {
    let root = std::env::temp_dir().join(format!("engines-zit-{}-count-ast-grep-{test}", std::process::id()));
    fs::create_dir(&root).unwrap();
    let folder = fs::canonicalize(root).unwrap();
    let text = format!(
        "version = 1\n[[rule]]\nid = \"count\"\nengine = \"ast-grep\"\ncheck = \"names.yml\"\nfix = \"f\"\nparams = {params}\n"
    );
    let mut file = RuleFile::parse(&text, &folder).unwrap();
    file.rules[0].embedded_text = Some(embedded);
    let seen = RefCell::new(None);
    let run = |cmd: &Cmd| {
        let at = cmd.args.iter().position(|arg| arg == "--rule").unwrap();
        let rule = PathBuf::from(&cmd.args[at + 1]);
        *seen.borrow_mut() = Some(Seen { text: fs::read_to_string(&rule).ok(), rule });
        answer()
    };
    let on_path = |_: &str| -> bool { panic!("count mode looks nothing up on PATH") };
    let host = Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None), base };
    let got = hits_ast_grep(&AST_GREP, &file.rules[0], &folder, &host);
    fs::remove_dir_all(&folder).unwrap();
    (got, seen.into_inner())
}

fn never_run() -> Result<Output, ProcError> {
    panic!("ast-grep must not run")
}

#[test]
fn count_mode_without_a_base_is_not_available_and_nothing_runs() {
    // The YAML names no param, so `count` reaching the fill would be refused as unused.
    let (got, seen) = run_count("no-base", PLAIN, "{ count = \"setup-vs-tests\" }", None, never_run);
    assert_eq!(got, Err(Stop::NotAvailable { why: ONLY_WITH_BASE_WHY.to_string() }));
    assert!(seen.is_none());
}

#[test]
fn output_that_is_not_an_answer_fails_with_its_exit_code_after_a_run_of_the_filled_yaml() {
    let params = "{ names = [\"foo\", \"a.b\"], count = \"setup-vs-tests\", shared_setup = [\"common/**\"] }";
    let answer = || Ok(Output { code: Some(8), stdout: b"not json\n".to_vec(), stderr: b"broken rule".to_vec() });
    let (got, seen) = run_count("garbage", NAMES, params, Some("abc123"), answer);
    assert_eq!(got, Err(Stop::Failed { code: Some(8), stderr: "broken rule".to_string() }));
    let seen = seen.expect("ast-grep ran");
    assert_eq!(seen.text.as_deref(), Some(NAMES_FILLED));
    assert!(!seen.rule.exists(), "{} is left behind", seen.rule.display());
}

#[test]
fn ast_grep_absent_in_count_mode_is_not_available_and_the_copy_is_gone() {
    let answer = || Err(ProcError::NotFound { program: "ast-grep".into() });
    let (got, seen) = run_count("absent", PLAIN, "{ count = \"setup-vs-tests\" }", Some("abc123"), answer);
    assert_eq!(got, Err(Stop::NotOnPath { program: "ast-grep".to_string() }));
    let seen = seen.expect("ast-grep ran");
    assert_eq!(seen.text.as_deref(), Some(PLAIN));
    assert!(!seen.rule.exists(), "{} is left behind", seen.rule.display());
}
