//! ZUT for run_plain.rs with a fake entry and a fake host: each step's stop, the arguments reaching the command,
//! the check run counted (and a refused one not), an unknown check, words about another check left to the reader,
//! output the reader cannot read, a threshold passed as an argument after the rule's config, a threshold out of range.
//! No engine runs and no file is written; the config file's run and a threshold's file are the ZIT's
//! (integration/temp_config_tests.rs, integration/run_plain_threshold_tests.rs).

use super::entry_cmd_tests::fake_entry;
use super::{run_plain, ConfigParams, Entry, ParamArg, Params, Pick, Reach, Threshold};
use crate::host::Host;
use crate::types::{Hit, ParseError, Stop};
use crate::ParamError;
use procio::{Cmd, Output, ProcError};
use rules::{RelPath, Rule, RuleFile};
use std::cell::{Cell, RefCell};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

const FOLDER: &str = "/no/such/engines-zut-folder";
static ALLOW: [ParamArg; 1] = [ParamArg { name: "allow", flag: "--allow=" }];

/// The fake entry with an `allow` param and a config option; it refuses the check `BAD`, says the check is unknown
/// when stderr holds "unknown check <check>", and reads stdout `hit` as one hit whose message names the check and the cwd.
fn lint() -> Entry {
    Entry {
        refuse: |check| (check == "BAD").then(|| "tm refuses BAD".to_string()),
        unknown: |out, check| {
            let stderr = String::from_utf8_lossy(&out.stderr);
            stderr.contains(&format!("unknown check {check}")).then(|| stderr.trim().to_string())
        },
        read: |out, check, cwd, _| match out.stdout.as_slice() {
            b"hit" => Ok(vec![Hit {
                path: RelPath::new("a.py"),
                line: Some(3),
                col: None,
                what: format!("{check} in {}", cwd.display()),
            }]),
            _ => Err(ParseError { why: "not lint output".to_string() }),
        },
        ..fake_entry(Pick::Arg("--select="), Params::Args(&ALLOW), Some("--config="))
    }
}

fn rule(check: &str, extra: &str) -> Rule {
    let text = format!("version = 1\n[[rule]]\nid = \"r\"\nengine = \"ruff\"\ncheck = \"{check}\"\nfix = \"f\"\n{extra}");
    RuleFile::parse(&text, Path::new(FOLDER)).unwrap().rules.remove(0)
}

/// `run_plain` of `entry` and `rule`, every run answering `answer`; also the commands run and the count left in
/// `raw_chars`.
fn run(entry: &Entry, rule: &Rule, answer: Output) -> (Result<Vec<Hit>, Stop>, Vec<Cmd>, Option<usize>) {
    let ran = RefCell::new(Vec::new());
    let host_run = |cmd: &Cmd| -> Result<Output, ProcError> {
        ran.borrow_mut().push(cmd.clone());
        Ok(answer.clone())
    };
    let raw_chars = Cell::new(None);
    let got = run_plain(entry, rule, Path::new(FOLDER), &Host { run: &host_run, on_path: &|_| true, raw_chars: &raw_chars });
    (got, ran.into_inner(), raw_chars.get())
}

fn answer(code: i32, stdout: &str, stderr: &str) -> Output {
    Output { code: Some(code), stdout: stdout.as_bytes().to_vec(), stderr: stderr.as_bytes().to_vec() }
}

fn failed(stderr: &str) -> Result<Vec<Hit>, Stop> {
    Err(Stop::Failed { code: None, stderr: stderr.to_string() })
}

#[test]
fn a_refused_check_fails_and_nothing_runs_or_is_counted() {
    let (got, ran, raw_chars) = run(&lint(), &rule("BAD", ""), answer(0, "hit", ""));
    assert_eq!(got, failed("tm refuses BAD"));
    assert!(ran.is_empty(), "{ran:?}");
    assert_eq!(raw_chars, None);
}

#[test]
fn params_the_entry_cannot_use_fail_with_the_param_why_and_nothing_runs() {
    let (got, ran, _) = run(&lint(), &rule("E711", "params = { allow = \"E1\" }\n"), answer(0, "hit", ""));
    assert_eq!(got, failed("lint param allow: must be a list of strings"));
    assert!(ran.is_empty(), "{ran:?}");
    let (got, ran, _) = run(&lint(), &rule("E711", "params = { deny = [\"E1\"] }\n"), answer(0, "hit", ""));
    assert_eq!(got, failed("lint param deny: a lint rule reads only the `allow` param"));
    assert!(ran.is_empty(), "{ran:?}");
}

#[test]
fn a_config_the_params_cannot_make_fails_and_nothing_runs() {
    let config = ConfigParams {
        option: "--config=",
        ext: "toml",
        text: |_| Err(ParamError::Unknown { name: "width".to_string() }),
    };
    let entry = Entry { params: Params::Config(config), ..lint() };
    let (got, ran, _) = run(&entry, &rule("E711", ""), answer(0, "hit", ""));
    assert_eq!(got, failed("lint param width: unknown"));
    assert!(ran.is_empty(), "{ran:?}");
}

#[test]
fn a_check_file_entry_has_no_standard_run_and_nothing_runs() {
    let entry = Entry { pick: Pick::File("--rule"), params: Params::Template, ..lint() };
    let (got, ran, _) = run(&entry, &rule("rules/x.yml", ""), answer(0, "hit", ""));
    assert_eq!(got, failed("lint has no standard run for a check file"));
    assert!(ran.is_empty(), "{ran:?}");
}

#[test]
fn the_check_params_and_config_reach_the_command_and_the_run_is_counted_and_read() {
    let rule = rule("E711", "config = \"lint.toml\"\nparams = { allow = [\"E1\", \"E2\"] }\n");
    let (got, ran, raw_chars) = run(&lint(), &rule, answer(0, "hit", ""));
    let args = ["check", "--select=E711", "--allow=E1", "--allow=E2", "--config=lint.toml", "."];
    assert_eq!(ran.len(), 1);
    assert_eq!(ran[0].program, OsString::from("lint"));
    assert_eq!(ran[0].args, args.map(OsString::from).to_vec());
    assert_eq!(ran[0].cwd, Some(PathBuf::from(FOLDER)));
    assert_eq!(raw_chars, Some(3));
    let hit = Hit { path: RelPath::new("a.py"), line: Some(3), col: None, what: format!("E711 in {FOLDER}") };
    assert_eq!(got, Ok(vec![hit]));
}

#[test]
fn an_unknown_check_stops_with_the_engines_words_after_a_counted_run() {
    let (got, ran, raw_chars) = run(&lint(), &rule("ZZZ999", ""), answer(2, "", "unknown check ZZZ999\n"));
    assert_eq!(got, Err(Stop::UnknownCheck { said: "unknown check ZZZ999".to_string() }));
    assert_eq!(ran.len(), 1);
    assert_eq!(raw_chars, Some(21));
}

#[test]
fn words_about_another_check_are_left_to_the_reader() {
    let (got, _, _) = run(&lint(), &rule("E711", ""), answer(2, "", "unknown check ZZZ999\n"));
    assert_eq!(got, Err(Stop::Failed { code: Some(2), stderr: "unknown check ZZZ999\n".to_string() }));
}

#[test]
fn output_the_reader_cannot_read_fails_with_the_exit_code() {
    let (got, _, _) = run(&lint(), &rule("E711", ""), answer(2, "garbage", ""));
    assert_eq!(got, Err(Stop::Failed { code: Some(2), stderr: "not lint output".to_string() }));
    let (got, _, _) = run(&lint(), &rule("E711", ""), answer(2, "garbage", "lint: bad flag"));
    assert_eq!(got, Err(Stop::Failed { code: Some(2), stderr: "lint: bad flag".to_string() }));
}

static COMPLEXITY: [Threshold; 1] = [Threshold {
    name: "max_complexity",
    check: "C901",
    max: 50,
    setting: |n| format!("lint.mccabe.max-complexity={n}"),
}];

/// `lint()` with the `allow` param and a `max_complexity` threshold passed as a `--config=` argument.
fn limited() -> Entry {
    let params = Params::Thresholds { args: &ALLOW, thresholds: &COMPLEXITY, reach: Reach::Arg("--config=") };
    Entry { params, ..lint() }
}

#[test]
fn a_threshold_follows_the_params_and_the_rules_config_in_one_run() {
    let rule = rule("C901", "config = \"lint.toml\"\nparams = { allow = [\"E1\"], max_complexity = 12 }\n");
    let (got, ran, raw_chars) = run(&limited(), &rule, answer(0, "hit", ""));
    let args =
        ["check", "--select=C901", "--allow=E1", "--config=lint.toml", "--config=lint.mccabe.max-complexity=12", "."];
    assert_eq!(ran.len(), 1);
    assert_eq!(ran[0].args, args.map(OsString::from).to_vec());
    assert!(ran[0].env.is_empty(), "{:?}", ran[0].env);
    assert_eq!(raw_chars, Some(3));
    let hit = Hit { path: RelPath::new("a.py"), line: Some(3), col: None, what: format!("C901 in {FOLDER}") };
    assert_eq!(got, Ok(vec![hit]));
}

#[test]
fn a_threshold_out_of_range_fails_and_nothing_runs() {
    let (got, ran, _) = run(&limited(), &rule("C901", "params = { max_complexity = 0 }\n"), answer(0, "hit", ""));
    assert_eq!(got, failed("lint param max_complexity: must be a whole number from 1 to 50"));
    assert!(ran.is_empty(), "{ran:?}");
}

#[test]
fn a_param_in_neither_list_fails_naming_the_args_and_the_thresholds() {
    let (got, ran, _) = run(&limited(), &rule("C901", "params = { deny = [\"E1\"] }\n"), answer(0, "hit", ""));
    assert_eq!(got, failed("lint param deny: a lint rule reads only the `allow`, `max_complexity` param"));
    assert!(ran.is_empty(), "{ran:?}");
}

#[test]
fn a_thresholds_entry_with_no_threshold_given_runs_as_args_with_the_rules_config() {
    let rule = rule("C901", "config = \"lint.toml\"\nparams = { allow = [\"E1\"] }\n");
    let (_, ran, _) = run(&limited(), &rule, answer(0, "hit", ""));
    let args = ["check", "--select=C901", "--allow=E1", "--config=lint.toml", "."];
    assert_eq!(ran[0].args, args.map(OsString::from).to_vec());
}
