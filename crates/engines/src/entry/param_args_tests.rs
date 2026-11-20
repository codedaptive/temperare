//! ZUT for param_args.rs: listed params to arguments in the list's order; an unlisted key, a param that is not a
//! list of strings and params that are not a table are `Bad`, with clippy's texts; an empty list refuses any param
//! as taking none; `Template` and `Config` give no arguments.

use super::entry_cmd_tests::fake_entry;
use super::{param_args, ConfigParams, Entry, ParamArg, Params, Pick};
use crate::ParamError;
use serde_json::json;
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
fn template_and_config_give_no_arguments_whatever_the_params() {
    let config = ConfigParams { option: "--config=", ext: "toml", text: |_| Ok(String::new()) };
    for params in [Params::Template, Params::Config(config)] {
        let entry = fake_entry(Pick::Arg("--select="), params, None);
        assert_eq!(param_args(&entry, &json!({"anything": 1})), Ok(Vec::new()), "{params:?}");
    }
}
