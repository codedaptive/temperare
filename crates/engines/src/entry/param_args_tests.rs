//! Unit test for param_args.rs: listed params to arguments in the list's order; an unlisted key, a param that is not a
//! list of strings and params that are not a table are `Bad`, with clippy's texts (`an` before an engine name led by a
//! vowel letter); an empty list refuses any param
//! as taking none; `Template` gives no arguments. A `Thresholds` entry's args work as `Args`, its
//! threshold keys give no arguments, and a key in neither list is `Bad` naming both lists. A `Flow` entry refuses a
//! key not in its names as `Bad` naming them, returns its check's error, and gives no arguments.

use super::entry_cmd_tests::fake_entry;
use super::{param_args, Entry, ParamArg, Params, Pick, Reach, Threshold};
use crate::ParamError;
use serde_json::{json, Map, Value};
use std::ffi::OsString;

static ALLOW: [ParamArg; 1] = [ParamArg { name: "allow", flag: "--allow=" }];
static ALLOW_DENY: [ParamArg; 2] =
    [ParamArg { name: "allow", flag: "--allow=" }, ParamArg { name: "deny", flag: "--deny=" }];

fn clippy_like() -> Entry {
    Entry { name: "clippy", ..fake_entry(Pick::Arg("--warn="), Params::Args(&ALLOW), None) }
}

fn bad(name: &str, why: &str) -> Result<Vec<OsString>, ParamError> {
    Err(ParamError::Bad { name: name.to_string(), why: why.to_string() })
}

fn os(args: &[&str]) -> Vec<OsString> {
    args.iter().map(OsString::from).collect()
}

#[test]
fn no_params_give_no_arguments() {
    assert_eq!(param_args(&clippy_like(), &json!({})), Ok(Vec::new()));
}

#[test]
fn each_item_is_one_argument_joined_to_its_flag_in_order() {
    let got = param_args(&clippy_like(), &json!({"allow": ["clippy::a", "-b"]}));
    assert_eq!(got, Ok(os(&["--allow=clippy::a", "--allow=-b"])));
}

#[test]
fn params_come_out_in_the_lists_order() {
    let entry = fake_entry(Pick::Arg("--select="), Params::Args(&ALLOW_DENY), None);
    let got = param_args(&entry, &json!({"deny": ["d"], "allow": ["a1", "a2"]}));
    assert_eq!(got, Ok(os(&["--allow=a1", "--allow=a2", "--deny=d"])));
}

#[test]
fn an_unlisted_key_is_bad_with_clippys_text() {
    let got = param_args(&clippy_like(), &json!({"alow": ["x"]}));
    assert_eq!(got, bad("alow", "a clippy rule reads only the `allow` param"));
}

#[test]
fn an_unlisted_key_is_bad_with_an_before_a_vowel_led_engine_name() {
    let named = |name, params| Entry { name, ..fake_entry(Pick::Arg("--rule="), params, None) };
    let rows: [(Entry, Result<Vec<OsString>, ParamError>); 3] = [
        (named("eslint", Params::Args(&[])), bad("foo", "an eslint rule takes no params")),
        (named("actionlint", Params::Args(&ALLOW)), bad("foo", "an actionlint rule reads only the `allow` param")),
        (named("ruff", Params::Args(&[])), bad("foo", "a ruff rule takes no params")),
    ];
    for (entry, want) in rows {
        assert_eq!(param_args(&entry, &json!({"foo": 1})), want, "{}", entry.name);
    }
}

#[test]
fn an_unlisted_key_names_every_listed_param() {
    let entry = fake_entry(Pick::Arg("--select="), Params::Args(&ALLOW_DENY), None);
    let got = param_args(&entry, &json!({"warn": []}));
    assert_eq!(got, bad("warn", "a lint rule reads only the `allow`, `deny` param"));
}

#[test]
fn a_param_that_is_not_a_list_of_strings_is_bad() {
    let not_list = "must be a list of strings";
    assert_eq!(param_args(&clippy_like(), &json!({"allow": "clippy::a"})), bad("allow", not_list));
    assert_eq!(param_args(&clippy_like(), &json!({"allow": ["clippy::a", 3]})), bad("allow", not_list));
}

#[test]
fn params_that_are_not_a_table_are_bad() {
    assert_eq!(param_args(&clippy_like(), &json!(["allow"])), bad("params", "not a table of named params"));
}

#[test]
fn an_empty_list_refuses_any_param_as_taking_no_params() {
    let entry = fake_entry(Pick::Arg("--select="), Params::Args(&[]), None);
    assert_eq!(param_args(&entry, &json!({"select": "E"})), bad("select", "a lint rule takes no params"));
}

#[test]
fn an_empty_list_with_no_params_gives_no_arguments() {
    let entry = fake_entry(Pick::Arg("--select="), Params::Args(&[]), None);
    assert_eq!(param_args(&entry, &json!({})), Ok(Vec::new()));
}

#[test]
fn template_gives_no_arguments_whatever_the_params() {
    let entry = fake_entry(Pick::Arg("--select="), Params::Template, None);
    assert_eq!(param_args(&entry, &json!({"anything": 1})), Ok(Vec::new()));
}

static NESTING: [Threshold; 1] =
    [Threshold { name: "max_nesting", check: "nesting_depth", max: 9, setting: |n| format!("n={n}") }];

fn thresholds_entry() -> Entry {
    let params = Params::Thresholds { args: &ALLOW, thresholds: &NESTING, reach: Reach::Arg("--config=") };
    fake_entry(Pick::Arg("--select="), params, None)
}

#[test]
fn a_thresholds_entrys_args_become_arguments_and_its_threshold_gives_none() {
    let got = param_args(&thresholds_entry(), &json!({"allow": ["a1", "a2"], "max_nesting": 3}));
    assert_eq!(got, Ok(os(&["--allow=a1", "--allow=a2"])));
    assert_eq!(param_args(&thresholds_entry(), &json!({"max_nesting": "not checked here"})), Ok(Vec::new()));
}

#[test]
fn a_key_in_neither_list_is_bad_naming_the_args_then_the_thresholds() {
    let got = param_args(&thresholds_entry(), &json!({"foo": 1}));
    assert_eq!(got, bad("foo", "a lint rule reads only the `allow`, `max_nesting` param"));
}

#[test]
fn a_thresholds_entry_with_both_lists_empty_takes_no_params() {
    let params = Params::Thresholds { args: &[], thresholds: &[], reach: Reach::Arg("--config=") };
    let entry = fake_entry(Pick::Arg("--select="), params, None);
    assert_eq!(param_args(&entry, &json!({"foo": 1})), bad("foo", "a lint rule takes no params"));
}

static FLOW_NAMES: [&str; 2] = ["max_ratio", "floor"];

fn no_floor_seven(params: &Map<String, Value>) -> Result<(), ParamError> {
    match params.get("floor") {
        Some(floor) if floor == &json!(7) => {
            Err(ParamError::Bad { name: "floor".to_string(), why: "seven is refused".to_string() })
        }
        _ => Ok(()),
    }
}

#[test]
fn a_flow_entry_refuses_unlisted_keys_then_runs_its_check_and_gives_no_arguments() {
    let entry = fake_entry(Pick::Arg("--select="), Params::Flow { names: &FLOW_NAMES, check: no_floor_seven }, None);
    let rows: [(&str, Value, Result<Vec<OsString>, ParamError>); 5] = [
        ("unlisted key", json!({"foo": 1}), bad("foo", "a lint rule reads only the `max_ratio`, `floor` param")),
        ("check refuses", json!({"max_ratio": 3, "floor": 7}), bad("floor", "seven is refused")),
        ("good table", json!({"max_ratio": 3, "floor": 2}), Ok(Vec::new())),
        ("empty table", json!({}), Ok(Vec::new())),
        ("not a table", json!(["floor"]), bad("params", "not a table of named params")),
    ];
    for (case, params, want) in rows {
        assert_eq!(param_args(&entry, &params), want, "{case}");
    }
}

#[test]
fn a_flow_entry_with_no_names_takes_no_params() {
    let entry = fake_entry(Pick::Arg("--select="), Params::Flow { names: &[], check: |_| Ok(()) }, None);
    assert_eq!(param_args(&entry, &json!({"floor": 2})), bad("floor", "a lint rule takes no params"));
}
