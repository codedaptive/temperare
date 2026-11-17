//! ZUT for parse_check_args.rs: the four flags, paths in order, the kind names, and the usage errors.

use std::path::PathBuf;

use super::parse_check_args::parse_check_args;
use super::{CheckArgs, UsageError};

fn parse(args: &[&str]) -> Result<CheckArgs, UsageError> {
    parse_check_args(&args.iter().map(|a| a.to_string()).collect::<Vec<_>>())
}

fn error(what: &str, fix: &str) -> UsageError {
    UsageError { what: what.to_string(), fix: fix.to_string() }
}

#[test]
fn nothing_is_the_default() {
    assert_eq!(
        parse(&[]),
        Ok(CheckArgs { rules: None, base: None, kind: None, json: false, paths: vec![] })
    );
}

#[test]
fn every_flag_and_paths_in_order() {
    assert_eq!(
        parse(&["b.rs", "--rules", "t.toml", "a.rs", "--base", "main", "--kind", "security", "--json", "c"]),
        Ok(CheckArgs {
            rules: Some(PathBuf::from("t.toml")),
            base: Some("main".to_string()),
            kind: Some(rules::Kind::Security),
            json: true,
            paths: vec![PathBuf::from("b.rs"), PathBuf::from("a.rs"), PathBuf::from("c")],
        })
    );
}

#[test]
fn kind_quality() {
    assert_eq!(parse(&["--kind", "quality"]).map(|a| a.kind), Ok(Some(rules::Kind::Quality)));
}

#[test]
fn a_bad_kind() {
    assert_eq!(
        parse(&["--kind", "style"]),
        Err(error("unknown kind 'style'", "Use --kind quality or --kind security."))
    );
}

#[test]
fn each_value_flag_needs_its_value() {
    assert_eq!(parse(&["--rules"]), Err(error("--rules needs a value", "Write --rules <path>.")));
    assert_eq!(parse(&["--base", "--json"]), Err(error("--base needs a value", "Write --base <rev>.")));
    assert_eq!(parse(&["--kind"]), Err(error("--kind needs a value", "Write --kind quality|security.")));
}

#[test]
fn an_unknown_flag_is_refused() {
    assert_eq!(
        parse(&["src", "-v"]),
        Err(error("unknown flag '-v' for tm check", "Run tm help to see the flags of tm check."))
    );
}

#[test]
fn flag_equals_value_is_an_unknown_flag() {
    assert_eq!(
        parse(&["--base=main"]),
        Err(error("unknown flag '--base=main' for tm check", "Run tm help to see the flags of tm check."))
    );
}
