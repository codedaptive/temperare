//! ZUT for ruff_entry.rs: the command `entry_cmd` builds from `RUFF`, the version read from fixed output, and
//! `run_plain(&RUFF, ..)` on a fake host: the command the host is given, findings read from captured output, garbage
//! output, an absent ruff, an unknown selector, the characters of the check run counted into the
//! host, the `max_complexity` threshold as an inline `--config` after the rule's config, and a threshold or param
//! refused before anything runs. Nothing runs and nothing is written. `DIR` is the folder path the captured fixture's absolute paths
//! start with (see parse_ruff_tests.rs); no folder is read, so it need not exist.

use super::RUFF;
use crate::entry::{entry_cmd, run_plain};
use crate::host::Host;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output, ProcError};
use rules::{Engine, RelPath, RuleFile};
use std::cell::{Cell, RefCell};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

const DIR: &str = "/work/project/cap-ruff";

const WITH_CONFIG: &str = "version = 1\n[[rule]]\nid = \"no-none-compare\"\nengine = \"ruff\"\ncheck = \"E711\"\nconfig = \"ruff.toml\"\nfix = \"f\"\n";
const NO_CONFIG: &str = "version = 1\n[[rule]]\nid = \"no-none-compare\"\nengine = \"ruff\"\ncheck = \"E711\"\nfix = \"f\"\n";

/// `run_plain(&RUFF, ..)` for the one rule in `text`, over `DIR`, with a host whose run answers `answer`; also the
/// command the run was given, if it was asked.
fn run_rule(text: &str, answer: impl Fn() -> Result<Output, ProcError>) -> (Result<Vec<Hit>, Stop>, Option<Cmd>) {
    let file = RuleFile::parse(text, Path::new(DIR)).unwrap();
    let seen = RefCell::new(None);
    let run = |cmd: &Cmd| {
        *seen.borrow_mut() = Some(cmd.clone());
        answer()
    };
    let on_path = |_: &str| -> bool { panic!("ruff's run looks nothing up on PATH") };
    let host = Host { run: &run, on_path: &on_path, raw_chars: &Cell::new(None), base: None };
    let got = run_plain(&RUFF, &file.rules[0], Path::new(DIR), &host);
    (got, seen.into_inner())
}

/// `run_plain(&RUFF, ..)` for the one rule in `text`, as `run_rule` runs it; also the count the host holds after it
/// and how many runs the host was asked for.
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
    let on_path = |_: &str| -> bool { panic!("ruff's run looks nothing up on PATH") };
    let raw_chars = Cell::new(None);
    let got = run_plain(&RUFF, &file.rules[0], Path::new(DIR), &Host { run: &run, on_path: &on_path, raw_chars: &raw_chars, base: None });
    (got, raw_chars.get(), runs.get())
}

fn captured() -> Result<Output, ProcError> {
    let stdout = include_bytes!("fixtures/e711_and_f401.json").to_vec();
    Ok(Output { code: Some(1), stdout, stderr: Vec::new() })
}

fn os(items: &[&str]) -> Vec<OsString> {
    items.iter().map(OsString::from).collect()
}

#[test]
fn the_entry_is_ruff_needing_ruff() {
    assert_eq!((RUFF.name, RUFF.engine, RUFF.program), ("ruff", Engine::Ruff, "ruff"));
    assert_eq!(RUFF.needs, ["ruff"]);
    assert_eq!(RUFF.never, None);
}

#[test]
fn without_a_config_it_checks_one_rule_as_json_over_the_folder_with_no_cache() {
    let cmd = entry_cmd(&RUFF, OsStr::new("E711"), &[], None, Path::new("/w/py"));
    assert_eq!(cmd.program, OsString::from("ruff"));
    assert_eq!(cmd.args, os(&["check", "--no-cache", "--output-format", "json", "--select=E711", "."]));
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/py")));
    assert!(cmd.env.is_empty() && cmd.env_remove.is_empty() && cmd.stdin.is_empty() && cmd.timeout.is_none());
}

#[test]
fn a_config_comes_before_the_folder_as_written() {
    let cmd = entry_cmd(&RUFF, OsStr::new("E711"), &[], Some("lint/ruff.toml"), Path::new("/w/py"));
    assert_eq!(
        cmd.args,
        os(&["check", "--no-cache", "--output-format", "json", "--select=E711", "--config=lint/ruff.toml", "."])
    );
}

#[test]
fn option_like_check_and_config_are_single_value_arguments() {
    let cmd = entry_cmd(&RUFF, OsStr::new("--fix"), &[], Some("--isolated"), Path::new("/w/py"));
    assert_eq!(cmd.args, os(&["check", "--no-cache", "--output-format", "json", "--select=--fix", "--config=--isolated", "."]));
}

#[test]
fn the_version_is_read_from_ruff_dash_dash_version() {
    let probe = RUFF.version.unwrap();
    assert_eq!(probe.args, ["--version"]);
    let out = Output { code: Some(0), stdout: b"ruff 0.17.4\n".to_vec(), stderr: Vec::new() };
    assert_eq!((probe.read)(&out), Some("0.17.4".to_string()));
}

#[test]
fn the_checked_rules_findings_come_through() {
    let (got, _) = run_rule(WITH_CONFIG, captured);
    let want = Hit {
        path: RelPath::new("example.py"),
        line: Some(3),
        col: Some(13),
        what: "Comparison to `None` should be `cond is None`".to_string(),
    };
    assert_eq!(got, Ok(vec![want]));
}

#[test]
fn the_rules_config_is_passed_and_ruff_runs_in_the_folder() {
    let cmd = run_rule(WITH_CONFIG, captured).1.unwrap();
    assert_eq!(
        cmd.args,
        os(&["check", "--no-cache", "--output-format", "json", "--select=E711", "--config=ruff.toml", "."])
    );
    assert_eq!(cmd.cwd, Some(PathBuf::from(DIR)));
}

#[test]
fn a_rule_with_no_config_passes_none() {
    let cmd = run_rule(NO_CONFIG, captured).1.unwrap();
    assert_eq!(cmd.args, os(&["check", "--no-cache", "--output-format", "json", "--select=E711", "."]));
}

#[test]
fn garbage_output_is_failed_with_the_exit_code_and_stderr() {
    // Hand-built, not captured.
    let (got, _) = run_rule(NO_CONFIG, || {
        Ok(Output { code: Some(2), stdout: b"garbage".to_vec(), stderr: b"ruff failed: bad config".to_vec() })
    });
    assert_eq!(got, Err(Stop::Failed { code: Some(2), stderr: "ruff failed: bad config".to_string() }));
}

#[test]
fn an_unknown_selector_is_an_unknown_check_with_ruffs_words() {
    // Hand-built in the shape real ruff prints for `--select=ZZZ999`.
    let rules = NO_CONFIG.replace("E711", "ZZZ999");
    let (got, _) = run_rule(&rules, || {
        Ok(Output {
            code: Some(2),
            stdout: Vec::new(),
            stderr: b"ruff failed\n  Cause: Unknown rule selector `ZZZ999` in `select` from the CLI\n".to_vec(),
        })
    });
    let said = "Cause: Unknown rule selector `ZZZ999` in `select` from the CLI".to_string();
    assert_eq!(got, Err(Stop::UnknownCheck { said }));
}

#[test]
fn an_unknown_selector_in_the_config_is_failed_not_blamed_on_the_check() {
    // Hand-built: ruff names a selector the rule's config file gives, not the check.
    let stderr = "ruff failed\n  Cause: Unknown rule selector `QQQ1` in `lint.select` in ruff.toml\n";
    let (got, _) = run_rule(WITH_CONFIG, || {
        Ok(Output { code: Some(2), stdout: Vec::new(), stderr: stderr.as_bytes().to_vec() })
    });
    assert_eq!(got, Err(Stop::Failed { code: Some(2), stderr: stderr.to_string() }));
}

#[test]
fn ruff_not_found_is_not_available() {
    let (got, _) = run_rule(NO_CONFIG, || Err(ProcError::NotFound { program: "ruff".into() }));
    assert_eq!(got, Err(Stop::NotOnPath { program: "ruff".to_string() }));
}

#[test]
fn the_check_run_counts_once_with_the_characters_of_stdout_and_stderr() {
    // "[]" is 2 characters; "warning: café\n" is 14 characters in 15 bytes.
    let (got, chars, runs) = run_counted(NO_CONFIG, || {
        Ok(Output { code: Some(0), stdout: b"[]".to_vec(), stderr: "warning: café\n".as_bytes().to_vec() })
    });
    assert_eq!(got, Ok(Vec::new()));
    assert_eq!((chars, runs), (Some(16), 1));
}

#[test]
fn a_run_failed_on_its_output_still_counts() {
    // "garbage" is 7 characters, "ruff failed: bad config" 23.
    let (got, chars, _) = run_counted(NO_CONFIG, || {
        Ok(Output { code: Some(2), stdout: b"garbage".to_vec(), stderr: b"ruff failed: bad config".to_vec() })
    });
    assert_eq!(got, Err(Stop::Failed { code: Some(2), stderr: "ruff failed: bad config".to_string() }));
    assert_eq!(chars, Some(30));
}

#[test]
fn ruff_not_found_counts_nothing() {
    let (_, chars, _) = run_counted(NO_CONFIG, || Err(ProcError::NotFound { program: "ruff".into() }));
    assert_eq!(chars, None);
}

#[test]
fn a_rule_with_an_unknown_param_is_failed_naming_max_complexity_and_nothing_runs() {
    let rules = format!("{NO_CONFIG}params = {{ select = \"E\" }}\n");
    let (got, chars, runs) = run_counted(&rules, captured);
    let stderr = "ruff param select: a ruff rule reads only the `max_complexity` param".to_string();
    assert_eq!(got, Err(Stop::Failed { code: None, stderr }));
    assert_eq!((chars, runs), (None, 0));
}

#[test]
fn an_unknown_param_foo_is_failed_and_nothing_runs() {
    let rules = format!("{NO_CONFIG}params = {{ foo = 1 }}\n");
    let (got, seen) = run_rule(&rules, captured);
    let stderr = "ruff param foo: a ruff rule reads only the `max_complexity` param".to_string();
    assert_eq!(got, Err(Stop::Failed { code: None, stderr }));
    assert!(seen.is_none());
}

#[test]
fn max_complexity_on_c901_runs_once_with_the_inline_threshold_config_before_the_folder() {
    let rules = format!("{}params = {{ max_complexity = 7 }}\n", NO_CONFIG.replace("E711", "C901"));
    let (_, _, runs) = run_counted(&rules, || Ok(Output { code: Some(0), stdout: b"[]".to_vec(), stderr: Vec::new() }));
    assert_eq!(runs, 1);
    let cmd = run_rule(&rules, captured).1.unwrap();
    assert_eq!(
        cmd.args,
        os(&[
            "check",
            "--no-cache",
            "--output-format",
            "json",
            "--select=C901",
            "--config=lint.mccabe.max-complexity=7",
            "."
        ])
    );
    assert!(cmd.env.is_empty(), "{:?}", cmd.env);
}

#[test]
fn max_complexity_comes_after_the_rules_config_so_it_wins_over_the_same_key() {
    let rules = format!("{}params = {{ max_complexity = 7 }}\n", WITH_CONFIG.replace("E711", "C901"));
    let cmd = run_rule(&rules, captured).1.unwrap();
    assert_eq!(
        cmd.args,
        os(&[
            "check",
            "--no-cache",
            "--output-format",
            "json",
            "--select=C901",
            "--config=ruff.toml",
            "--config=lint.mccabe.max-complexity=7",
            "."
        ])
    );
}

#[test]
fn max_complexity_on_another_check_is_failed_and_nothing_runs() {
    let rules = format!("{NO_CONFIG}params = {{ max_complexity = 7 }}\n");
    let (got, seen) = run_rule(&rules, captured);
    let stderr = "ruff param max_complexity: read only with check `C901`".to_string();
    assert_eq!(got, Err(Stop::Failed { code: None, stderr }));
    assert!(seen.is_none());
}

#[test]
fn max_complexity_outside_1_to_1000_is_failed_and_nothing_runs() {
    let c901 = NO_CONFIG.replace("E711", "C901");
    for n in ["0", "1001", "-3", "2.5", "\"7\""] {
        let (got, seen) = run_rule(&format!("{c901}params = {{ max_complexity = {n} }}\n"), captured);
        let stderr = "ruff param max_complexity: must be a whole number from 1 to 1000".to_string();
        assert_eq!(got, Err(Stop::Failed { code: None, stderr }), "max_complexity = {n}");
        assert!(seen.is_none(), "max_complexity = {n}");
    }
}

#[test]
fn max_complexity_at_1_and_1000_runs() {
    let c901 = NO_CONFIG.replace("E711", "C901");
    for (n, arg) in [("1", "--config=lint.mccabe.max-complexity=1"), ("1000", "--config=lint.mccabe.max-complexity=1000")] {
        let cmd = run_rule(&format!("{c901}params = {{ max_complexity = {n} }}\n"), captured).1.unwrap();
        assert_eq!(cmd.args[cmd.args.len() - 2], OsString::from(arg));
    }
}
