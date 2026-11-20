//! ZUT for swiftlint_entry.rs: the command `entry_cmd` builds from `SWIFTLINT`, the version read from fixed output,
//! and `run_plain(&SWIFTLINT, ..)` on a fake host: the command the host is given, findings read from captured output,
//! garbage output, an unknown rule, a `custom_rules` check and a rule with a param refused before anything runs, an
//! absent swiftlint, and the characters of the check run counted into the host. Nothing runs. `DIR` is the folder
//! path the captured fixture's absolute paths start with (see parse_swiftlint_tests.rs); no folder is read, so it
//! need not exist.

use super::SWIFTLINT;
use crate::entry::{entry_cmd, run_plain};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::{Engine, RelPath, RuleFile};
use std::cell::{Cell, RefCell};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

const DIR: &str = "/work/project/cap-swiftlint";

const WITH_CONFIG: &str = "version = 1\n[[rule]]\nid = \"no-force-cast\"\nengine = \"swiftlint\"\ncheck = \"force_cast\"\nconfig = \"two.yml\"\nfix = \"f\"\n";
const NO_CONFIG: &str = "version = 1\n[[rule]]\nid = \"no-force-cast\"\nengine = \"swiftlint\"\ncheck = \"force_cast\"\nfix = \"f\"\n";
const UNKNOWN_SAID: &str = "warning: 'no_such_rule' is not a valid rule identifier";

/// `run_plain(&SWIFTLINT, ..)` for the one rule in `text`, over `DIR`, with a host whose run answers `answer`; also
/// the command the run was given, if it was asked.
fn run_rule(text: &str, answer: impl Fn() -> Result<Output, ProcError>) -> (Result<Vec<Hit>, Stop>, Option<Cmd>) {
    let file = RuleFile::parse(text, Path::new(DIR)).unwrap();
    let seen = RefCell::new(None);
    let run = |cmd: &Cmd| {
        *seen.borrow_mut() = Some(cmd.clone());
        answer()
    };
    let on_path = |_: &str| -> bool { panic!("swiftlint's run looks nothing up on PATH") };
    let host = Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None) };
    let got = run_plain(&SWIFTLINT, &file.rules[0], Path::new(DIR), &host);
    (got, seen.into_inner())
}

/// `run_plain(&SWIFTLINT, ..)` for the one rule in `text`, as `run_rule` runs it; also the count the host holds after
/// it and how many runs the host was asked for.
fn run_counted(
    text: &str,
    answer: impl Fn() -> Result<Output, ProcError>,
) -> (Result<Vec<Hit>, Stop>, Option<usize>, usize) {
    let file = RuleFile::parse(text, Path::new(DIR)).unwrap();
    let runs = Cell::new(0);
    let run = |_: &Cmd| {
        runs.set(runs.get() + 1);
        answer()
    };
    let on_path = |_: &str| -> bool { panic!("swiftlint's run looks nothing up on PATH") };
    let raw_chars = Cell::new(None);
    let host = Host { run: &run, on_path: &on_path, raw_chars: &raw_chars };
    let got = run_plain(&SWIFTLINT, &file.rules[0], Path::new(DIR), &host);
    (got, raw_chars.get(), runs.get())
}

fn captured() -> Result<Output, ProcError> {
    let stdout = include_bytes!("fixtures/force_cast_and_others.json").to_vec();
    Ok(Output { code: Some(2), stdout, stderr: Vec::new() })
}

fn os(items: &[&str]) -> Vec<OsString> {
    items.iter().map(OsString::from).collect()
}

#[test]
fn the_entry_is_swiftlint_needing_swiftlint() {
    assert_eq!((SWIFTLINT.name, SWIFTLINT.engine, SWIFTLINT.program), ("swiftlint", Engine::SwiftLint, "swiftlint"));
    assert_eq!(SWIFTLINT.needs, ["swiftlint"]);
    assert_eq!(SWIFTLINT.never, None);
}

#[test]
fn without_a_config_it_lints_one_rule_as_json_in_the_folder() {
    let cmd = entry_cmd(&SWIFTLINT, OsStr::new("force_cast"), &[], None, Path::new("/w/app"));
    assert_eq!(cmd.program, OsString::from("swiftlint"));
    assert_eq!(cmd.args, os(&["lint", "--reporter", "json", "--quiet", "--only-rule=force_cast"]));
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/app")));
    assert!(cmd.env.is_empty() && cmd.env_remove.is_empty() && cmd.stdin.is_empty() && cmd.timeout.is_none());
}

#[test]
fn a_config_is_passed_last_as_written() {
    let cmd = entry_cmd(&SWIFTLINT, OsStr::new("force_cast"), &[], Some("lint/.swiftlint.yml"), Path::new("/w/app"));
    assert_eq!(
        cmd.args,
        os(&["lint", "--reporter", "json", "--quiet", "--only-rule=force_cast", "--config=lint/.swiftlint.yml"])
    );
}

#[test]
fn option_like_check_and_config_are_single_value_arguments() {
    let cmd = entry_cmd(&SWIFTLINT, OsStr::new("--help"), &[], Some("--isolated"), Path::new("/w/app"));
    assert_eq!(cmd.args, os(&["lint", "--reporter", "json", "--quiet", "--only-rule=--help", "--config=--isolated"]));
}

#[test]
fn the_version_is_read_from_swiftlint_version() {
    let probe = SWIFTLINT.version.unwrap();
    assert_eq!(probe.args, ["version"]);
    let out = Output { code: Some(0), stdout: b"0.59.1\n".to_vec(), stderr: Vec::new() };
    assert_eq!((probe.read)(&out), Some("0.59.1".to_string()));
}

#[test]
fn the_checked_rules_findings_come_through() {
    let (got, _) = run_rule(WITH_CONFIG, captured);
    let hit = |path: &str, line: u32, col: u32| Hit {
        path: RelPath::new(path),
        line: Some(line),
        col: Some(col),
        what: "Force casts should be avoided".to_string(),
    };
    assert_eq!(got, Ok(vec![hit("Example.swift", 2, 22), hit("NoNewline.swift", 1, 11)]));
}

#[test]
fn the_rules_config_is_passed_and_swiftlint_runs_in_the_folder() {
    let cmd = run_rule(WITH_CONFIG, captured).1.unwrap();
    assert_eq!(cmd.args, os(&["lint", "--reporter", "json", "--quiet", "--only-rule=force_cast", "--config=two.yml"]));
    assert_eq!(cmd.cwd, Some(PathBuf::from(DIR)));
}

#[test]
fn a_rule_with_no_config_passes_none() {
    let cmd = run_rule(NO_CONFIG, captured).1.unwrap();
    assert_eq!(cmd.args, os(&["lint", "--reporter", "json", "--quiet", "--only-rule=force_cast"]));
}

#[test]
fn garbage_output_is_failed_with_the_exit_code_and_stderr() {
    // Hand-built, not captured.
    let (got, _) = run_rule(NO_CONFIG, || {
        Ok(Output { code: Some(1), stdout: b"garbage".to_vec(), stderr: b"Error: bad config".to_vec() })
    });
    assert_eq!(got, Err(Stop::Failed { code: Some(1), stderr: "Error: bad config".to_string() }));
}

#[test]
fn unknown_rule_is_an_unknown_check_with_the_warning_only() {
    let rules = NO_CONFIG.replace("force_cast", "no_such_rule");
    let (got, _) = run_rule(&rules, || {
        Ok(Output {
            code: Some(0),
            stdout: b"[\n\n]".to_vec(),
            stderr: b"warning: 'no_such_rule' is not a valid rule identifier\nValid rule identifiers:\nforce_cast\nforce_try\nline_length\n".to_vec(),
        })
    });
    assert_eq!(got, Err(Stop::UnknownCheck { said: UNKNOWN_SAID.to_string() }));
}

#[test]
fn a_warning_about_another_id_is_left_to_the_reader() {
    // Hand-built: swiftlint warns about an id the rule's config names, and reads the check's run as clean.
    let (got, _) = run_rule(WITH_CONFIG, || {
        Ok(Output {
            code: Some(0),
            stdout: b"[\n\n]".to_vec(),
            stderr: b"warning: 'from_config_rule' is not a valid rule identifier\n".to_vec(),
        })
    });
    assert_eq!(got, Ok(Vec::new()));
}

#[test]
fn custom_rules_check_is_refused_without_running_swiftlint() {
    let rules = NO_CONFIG.replace("force_cast", "custom_rules");
    let (got, seen) = run_rule(&rules, || Ok(Output { code: Some(0), stdout: b"[]".to_vec(), stderr: Vec::new() }));
    assert_eq!(got, Err(Stop::Failed { code: None, stderr: "swiftlint check `custom_rules` names no rule swiftlint runs under --only-rule: name the custom rule's own identifier".to_string() }));
    assert!(seen.is_none());
}

#[test]
fn swiftlint_not_found_is_not_available() {
    let (got, _) = run_rule(NO_CONFIG, || Err(ProcError::NotFound { program: "swiftlint".into() }));
    assert_eq!(got, Err(Stop::NotAvailable { why: "swiftlint not found on PATH".to_string() }));
}

#[test]
fn the_check_run_counts_once_with_the_characters_of_stdout_and_stderr() {
    // "[]" is 2 characters; "Done linting: café\n" is 19 characters in 20 bytes.
    let (got, chars, runs) = run_counted(NO_CONFIG, || {
        Ok(Output { code: Some(0), stdout: b"[]".to_vec(), stderr: "Done linting: café\n".as_bytes().to_vec() })
    });
    assert_eq!(got, Ok(Vec::new()));
    assert_eq!((chars, runs), (Some(21), 1));
}

#[test]
fn an_unknown_rule_stopped_after_its_run_still_counts() {
    // "[\n\n]" is 4 characters; the warning line with its newline is 55.
    let rules = NO_CONFIG.replace("force_cast", "no_such_rule");
    let (got, chars, _) = run_counted(&rules, || {
        Ok(Output {
            code: Some(0),
            stdout: b"[\n\n]".to_vec(),
            stderr: b"warning: 'no_such_rule' is not a valid rule identifier\n".to_vec(),
        })
    });
    assert_eq!(got, Err(Stop::UnknownCheck { said: UNKNOWN_SAID.to_string() }));
    assert_eq!(chars, Some(59));
}

#[test]
fn a_custom_rules_check_refused_before_its_run_counts_nothing() {
    let rules = NO_CONFIG.replace("force_cast", "custom_rules");
    let (_, chars, runs) = run_counted(&rules, captured);
    assert_eq!((chars, runs), (None, 0));
}

#[test]
fn swiftlint_not_found_counts_nothing() {
    let (_, chars, _) = run_counted(NO_CONFIG, || Err(ProcError::NotFound { program: "swiftlint".into() }));
    assert_eq!(chars, None);
}

#[test]
fn a_rule_with_a_param_is_failed_as_taking_none_and_nothing_runs() {
    let rules = format!("{NO_CONFIG}params = {{ only = [\"force_try\"] }}\n");
    let (got, chars, runs) = run_counted(&rules, captured);
    let stderr = "swiftlint param only: a swiftlint rule takes no params".to_string();
    assert_eq!(got, Err(Stop::Failed { code: None, stderr }));
    assert_eq!((chars, runs), (None, 0));
}
