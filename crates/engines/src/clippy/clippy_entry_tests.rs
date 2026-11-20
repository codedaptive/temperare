//! ZUT for clippy_entry.rs: what `CLIPPY` needs, the command `entry_cmd` builds from it with and without allows, the
//! `allow` param as `param_args` reads it, the `max_nesting` threshold as `threshold_of` reads it, and the version read
//! from fixed output. Nothing runs.

use super::CLIPPY;
use crate::entry::{entry_cmd, param_args, threshold_of, Params, Reach};
use crate::ParamError;
use procio::Output;
use rules::Engine;
use serde_json::json;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

fn os(args: &[&str]) -> Vec<OsString> {
    args.iter().map(OsString::from).collect()
}

fn bad(name: &str, why: &str) -> Result<Vec<OsString>, ParamError> {
    Err(ParamError::Bad { name: name.to_string(), why: why.to_string() })
}

#[test]
fn the_entry_is_clippy_through_cargo_needing_both_clippy_programs() {
    assert_eq!((CLIPPY.name, CLIPPY.engine, CLIPPY.program), ("clippy", Engine::Clippy, "cargo"));
    assert_eq!(CLIPPY.needs, ["cargo-clippy", "clippy-driver"]);
    assert_eq!(CLIPPY.never, None);
}

#[test]
fn clippy_refuses_no_clippy_lint_and_reads_no_unknown_words() {
    assert_eq!((CLIPPY.refuse)("clippy::bool_comparison"), None);
    let out = Output { code: Some(101), stdout: Vec::new(), stderr: b"error: unknown lint: `x`".to_vec() };
    assert_eq!((CLIPPY.unknown)(&out, "x"), None);
}

#[test]
fn runs_cargo_clippy_json_over_all_targets_with_the_lint_warned_in_the_cargo_root() {
    let cmd = entry_cmd(&CLIPPY, OsStr::new("clippy::bool_comparison"), &[], None, Path::new("/w/crate"));
    assert_eq!(cmd.program, OsString::from("cargo"));
    let want = os(&["clippy", "--all-targets", "--message-format=json", "--quiet", "--", "--warn=clippy::bool_comparison"]);
    assert_eq!(cmd.args, want);
    assert_eq!(cmd.cwd, Some(PathBuf::from("/w/crate")));
    assert!(cmd.env.is_empty());
    assert!(cmd.env_remove.is_empty());
    assert!(cmd.stdin.is_empty());
    assert_eq!(cmd.timeout, None);
}

#[test]
fn each_allow_follows_the_check_as_an_allow_value_in_order() {
    let allows = param_args(&CLIPPY, &json!({"allow": ["clippy::result_large_err", "clippy::module_inception"]})).unwrap();
    let cmd = entry_cmd(&CLIPPY, OsStr::new("warnings"), &allows, None, Path::new("/w/crate"));
    let want = os(&[
        "clippy",
        "--all-targets",
        "--message-format=json",
        "--quiet",
        "--",
        "--warn=warnings",
        "--allow=clippy::result_large_err",
        "--allow=clippy::module_inception",
    ]);
    assert_eq!(cmd.args, want);
}

#[test]
fn a_rule_config_is_not_passed() {
    let cmd = entry_cmd(&CLIPPY, OsStr::new("unused_variables"), &[], Some("clippy.toml"), Path::new("/w/crate"));
    assert_eq!(cmd.args.last(), Some(&OsString::from("--warn=unused_variables")));
}

#[test]
fn option_like_check_and_allow_are_single_value_arguments() {
    let allows = param_args(&CLIPPY, &json!({"allow": ["--cap-lints"]})).unwrap();
    let cmd = entry_cmd(&CLIPPY, OsStr::new("--help"), &allows, None, Path::new("/w/crate"));
    assert_eq!(cmd.args, os(&["clippy", "--all-targets", "--message-format=json", "--quiet", "--", "--warn=--help", "--allow=--cap-lints"]));
}

#[test]
fn empty_params_allow_nothing() {
    assert_eq!(param_args(&CLIPPY, &json!({})), Ok(vec![]));
    assert_eq!(param_args(&CLIPPY, &json!({"allow": []})), Ok(vec![]));
}

#[test]
fn a_key_other_than_allow_is_bad_and_named() {
    let got = param_args(&CLIPPY, &json!({"alow": ["clippy::module_inception"]}));
    assert_eq!(got, bad("alow", "a clippy rule reads only the `allow`, `max_nesting` param"));
}

#[test]
fn an_unknown_key_beside_allow_is_still_bad() {
    let got = param_args(&CLIPPY, &json!({"allow": ["x"], "deny": ["y"]}));
    assert_eq!(got, bad("deny", "a clippy rule reads only the `allow`, `max_nesting` param"));
}

#[test]
fn an_allow_that_is_a_string_or_holds_a_non_string_is_bad() {
    assert_eq!(param_args(&CLIPPY, &json!({"allow": "x"})), bad("allow", "must be a list of strings"));
    assert_eq!(param_args(&CLIPPY, &json!({"allow": ["x", 2]})), bad("allow", "must be a list of strings"));
}

#[test]
fn params_that_are_not_a_table_are_bad() {
    assert_eq!(param_args(&CLIPPY, &json!(["x"])), bad("params", "not a table of named params"));
}

#[test]
fn the_version_is_read_from_cargo_clippy_dash_dash_version() {
    let probe = CLIPPY.version.unwrap();
    assert_eq!(probe.args, ["clippy", "--version"]);
    let out = Output { code: Some(0), stdout: b"clippy 0.1.95 (abc1234 2025-11-30)\n".to_vec(), stderr: Vec::new() };
    assert_eq!((probe.read)(&out), Some("0.1.95".to_string()));
}

#[test]
fn max_nesting_is_the_one_threshold_for_excessive_nesting_up_to_1000_written_as_clippy_toml_in_clippy_conf_dir() {
    let Params::Thresholds { args, thresholds, reach } = CLIPPY.params else { panic!("clippy takes thresholds") };
    assert_eq!(args.iter().map(|arg| (arg.name, arg.flag)).collect::<Vec<_>>(), [("allow", "--allow=")]);
    assert_eq!(thresholds.len(), 1);
    let threshold = thresholds[0];
    assert_eq!((threshold.name, threshold.check, threshold.max), ("max_nesting", "clippy::excessive_nesting", 1000));
    assert_eq!((threshold.setting)(3), "excessive-nesting-threshold = 3\n");
    assert_eq!(reach, Reach::Dir { env: "CLIPPY_CONF_DIR", file: "clippy.toml" });
}

#[test]
fn max_nesting_is_read_for_the_check_written_with_dashes_and_gives_no_argument() {
    let params = json!({"allow": ["x"], "max_nesting": 1000});
    let found = threshold_of(&CLIPPY, "clippy::excessive-nesting", &params).unwrap().map(|(t, n)| (t.name, n));
    assert_eq!(found, Some(("max_nesting", 1000)));
    assert_eq!(param_args(&CLIPPY, &params), Ok(os(&["--allow=x"])));
}

#[test]
fn max_nesting_out_of_range_or_on_another_check_is_bad() {
    let range = "must be a whole number from 1 to 1000";
    let refused = |check: &str, n: i64| threshold_of(&CLIPPY, check, &json!({"max_nesting": n})).map(|_| ());
    let bad_threshold = |why: &str| Err(ParamError::Bad { name: "max_nesting".to_string(), why: why.to_string() });
    assert_eq!(refused("clippy::excessive_nesting", 0), bad_threshold(range));
    assert_eq!(refused("clippy::excessive_nesting", 1001), bad_threshold(range));
    assert_eq!(refused("clippy::bool_comparison", 3), bad_threshold("read only with check `clippy::excessive_nesting`"));
}
