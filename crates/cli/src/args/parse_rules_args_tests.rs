//! ZUT for parse_rules_args.rs: `--rules <path>`, `--json`, `--loosened` with `--base <rev>`; anything else is a
//! usage error.

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
    assert_eq!(parse(&[]), Ok(RulesArgs::default()));
}

#[test]
fn rules_and_json() {
    assert_eq!(
        parse(&["--json", "--rules", "conf/t.toml"]),
        Ok(RulesArgs { rules: Some(PathBuf::from("conf/t.toml")), json: true, ..RulesArgs::default() })
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

#[test]
fn loosened_with_base_in_either_order() {
    let want = |json| RulesArgs { base: Some("main".to_string()), loosened: true, json, ..RulesArgs::default() };
    let rows: [(&[&str], RulesArgs); 4] = [
        (&["--loosened", "--base", "main"], want(false)),
        (&["--base", "main", "--loosened"], want(false)),
        (&["--json", "--base", "main", "--loosened"], want(true)),
        (&["--loosened", "--json", "--base", "main"], want(true)),
    ];
    for (args, expected) in rows {
        assert_eq!(parse(args), Ok(expected), "{args:?}");
    }
}

#[test]
fn loosened_and_base_each_need_the_other() {
    let no_base = UsageError {
        what: "tm rules --loosened needs --base <rev>".to_string(),
        fix: "Pass --base <rev>, the commit to compare with.".to_string(),
    };
    let no_loosened = UsageError {
        what: "--base is only for tm rules --loosened".to_string(),
        fix: "Add --loosened, or leave --base out.".to_string(),
    };
    let base_needs_a_value =
        UsageError { what: "--base needs a value".to_string(), fix: "Write --base <rev>.".to_string() };
    let rows: [(&[&str], UsageError); 6] = [
        (&["--loosened"], no_base.clone()),
        (&["--json", "--loosened", "--rules", "t.toml"], no_base),
        (&["--base", "main"], no_loosened.clone()),
        (&["--base", "main", "--json"], no_loosened),
        (&["--loosened", "--base"], base_needs_a_value.clone()),
        (&["--base", "--loosened"], base_needs_a_value),
    ];
    for (args, expected) in rows {
        assert_eq!(parse(args), Err(expected), "{args:?}");
    }
}
