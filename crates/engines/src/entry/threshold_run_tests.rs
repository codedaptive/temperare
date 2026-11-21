//! Unit tests for threshold_run.rs, the cases that write nothing: an `Arg` reach gives the rule's config then the
//! threshold; no config when the rule or the entry gives none; a refused threshold is `param_why`'s text; no
//! threshold, and an entry without thresholds, give `None`. The `File` and `Dir` reaches are tested in
//! integration/threshold_run_tests.rs.

use super::entry_cmd_tests::fake_entry;
use super::{threshold_run, Entry, Params, Pick, Reach, Threshold};
use rules::{Rule, RuleFile};
use std::ffi::OsString;
use std::path::Path;

const FOLDER: &str = "/no/such/engines-zut-folder";
static COMPLEXITY: [Threshold; 1] =
    [Threshold { name: "max_complexity", check: "C901", max: 50, setting: |n| format!("lint.max-complexity={n}") }];

fn limited(reach: Reach, config: Option<&'static str>) -> Entry {
    let params = Params::Thresholds { args: &[], thresholds: &COMPLEXITY, reach };
    fake_entry(Pick::Arg("--select="), params, config)
}

fn rule(check: &str, extra: &str) -> Rule {
    let text = format!("version = 1\n[[rule]]\nid = \"r\"\nengine = \"ruff\"\ncheck = \"{check}\"\nfix = \"f\"\n{extra}");
    RuleFile::parse(&text, Path::new(FOLDER)).unwrap().rules.remove(0)
}

fn os(args: &[&str]) -> Vec<OsString> {
    args.iter().map(OsString::from).collect()
}

#[test]
fn an_arg_reach_gives_the_rules_config_then_the_threshold() {
    let rule = rule("C901", "config = \"lint.toml\"\nparams = { max_complexity = 12 }\n");
    let run = threshold_run(&limited(Reach::Arg("--config="), Some("--config=")), &rule, Path::new(FOLDER));
    let run = run.unwrap().unwrap();
    assert_eq!(run.args, os(&["--config=lint.toml", "--config=lint.max-complexity=12"]));
    assert!(run.env.is_empty(), "{:?}", run.env);
}

#[test]
fn no_config_argument_when_the_rule_or_the_entry_gives_none() {
    let entry = limited(Reach::Arg("--set="), Some("--config="));
    let run = threshold_run(&entry, &rule("C901", "params = { max_complexity = 3 }\n"), Path::new(FOLDER));
    assert_eq!(run.unwrap().unwrap().args, os(&["--set=lint.max-complexity=3"]));
    let entry = limited(Reach::Arg("--set="), None);
    let rule = rule("C901", "config = \"lint.toml\"\nparams = { max_complexity = 3 }\n");
    assert_eq!(threshold_run(&entry, &rule, Path::new(FOLDER)).unwrap().unwrap().args, os(&["--set=lint.max-complexity=3"]));
}

#[test]
fn a_refused_threshold_is_the_param_why() {
    let entry = limited(Reach::Arg("--config="), Some("--config="));
    let got = threshold_run(&entry, &rule("C901", "params = { max_complexity = 0 }\n"), Path::new(FOLDER));
    assert_eq!(got.unwrap_err(), "lint param max_complexity: must be a whole number from 1 to 50");
    let got = threshold_run(&entry, &rule("E711", "params = { max_complexity = 5 }\n"), Path::new(FOLDER));
    assert_eq!(got.unwrap_err(), "lint param max_complexity: read only with check `C901`");
}

#[test]
fn a_refused_threshold_writes_nothing_for_a_file_reach_either() {
    let entry = limited(Reach::File { option: "--config=", ext: "yml", own: None }, None);
    let got = threshold_run(&entry, &rule("C901", "params = { max_complexity = 51 }\n"), Path::new(FOLDER));
    assert_eq!(got.unwrap_err(), "lint param max_complexity: must be a whole number from 1 to 50");
}

#[test]
fn no_threshold_given_is_none() {
    let entry = limited(Reach::Arg("--config="), Some("--config="));
    let got = threshold_run(&entry, &rule("C901", "config = \"lint.toml\"\n"), Path::new(FOLDER));
    assert!(got.unwrap().is_none());
}

#[test]
fn an_entry_without_thresholds_is_none() {
    let entry = fake_entry(Pick::Arg("--select="), Params::Args(&[]), Some("--config="));
    let got = threshold_run(&entry, &rule("C901", "params = { max_complexity = 5 }\n"), Path::new(FOLDER));
    assert!(got.unwrap().is_none());
}
