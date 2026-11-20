//! Unit tests for parse_rules_args.rs: `--rules <path>` and `--json`; anything else is a usage error.

use std::path::PathBuf;

use super::parse_rules_args::parse_rules_args;
use super::{RulesArgs, UsageError};

fn parse(args: &[&str]) -> Result<RulesArgs, UsageError> {
    parse_rules_args(&args.iter().map(|a| a.to_string()).collect::<Vec<_>>())
}

fn rules_error(what: &str) -> UsageError {
    UsageError { what: what.to_string(), fix: "Run tm help to see the flags of tm rules.".to_string() }
}

#[test]
fn nothing_is_the_default() {
    assert_eq!(parse(&[]), Ok(RulesArgs { rules: None, json: false }));
}

#[test]
fn rules_and_json() {
    assert_eq!(
        parse(&["--json", "--rules", "conf/t.toml"]),
        Ok(RulesArgs { rules: Some(PathBuf::from("conf/t.toml")), json: true })
    );
}

#[test]
fn rules_without_a_value() {
    let want = UsageError { what: "--rules needs a value".to_string(), fix: "Write --rules <path>.".to_string() };
    assert_eq!(parse(&["--rules"]), Err(want.clone()));
    assert_eq!(parse(&["--rules", "--json"]), Err(want));
}

#[test]
fn a_positional_argument_is_refused() {
    assert_eq!(parse(&["src"]), Err(rules_error("unexpected argument 'src' for tm rules")));
}

#[test]
fn check_flags_are_unknown_here() {
    assert_eq!(parse(&["--base", "main"]), Err(rules_error("unknown flag '--base' for tm rules")));
    assert_eq!(parse(&["--kind", "security"]), Err(rules_error("unknown flag '--kind' for tm rules")));
}

#[test]
fn an_unknown_flag_is_refused() {
    assert_eq!(parse(&["--frob"]), Err(rules_error("unknown flag '--frob' for tm rules")));
}

#[test]
fn flag_equals_value_is_an_unknown_flag() {
    assert_eq!(parse(&["--rules=t.toml"]), Err(rules_error("unknown flag '--rules=t.toml' for tm rules")));
}
