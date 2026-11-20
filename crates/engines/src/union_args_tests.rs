//! ZUT for union_args.rs: ruff's one comma list, one argument per check for the other arg picks, none for the rest.

use crate::ast_grep::AST_GREP;
use crate::clippy::CLIPPY;
use crate::ruff::RUFF;
use crate::swiftlint::SWIFTLINT;
use crate::tessera::TESSERA;
use crate::union_args::union_args;
use std::ffi::OsString;

fn args(of: &[&str]) -> Vec<OsString> {
    of.iter().map(OsString::from).collect()
}

#[test]
fn ruff_joins_the_checks_into_one_select() {
    assert_eq!(union_args(&RUFF, &["E711", "F401", "E7"]), args(&["--select=E711,F401,E7"]));
}

#[test]
fn ruff_drops_a_repeated_check() {
    assert_eq!(union_args(&RUFF, &["E711", "F401", "E711"]), args(&["--select=E711,F401"]));
}

#[test]
fn clippy_gives_one_warn_per_check() {
    assert_eq!(
        union_args(&CLIPPY, &["needless_return", "dead_code"]),
        args(&["--warn=needless_return", "--warn=dead_code"])
    );
}

#[test]
fn swiftlint_gives_one_only_rule_per_distinct_check_in_order() {
    assert_eq!(
        union_args(&SWIFTLINT, &["line_length", "force_cast", "line_length"]),
        args(&["--only-rule=line_length", "--only-rule=force_cast"])
    );
}

#[test]
fn a_file_pick_gives_no_arguments() {
    assert_eq!(union_args(&AST_GREP, &["rules/no-eval.yml"]), Vec::<OsString>::new());
}

#[test]
fn no_pick_gives_no_arguments() {
    assert_eq!(union_args(&TESSERA, &["slop"]), Vec::<OsString>::new());
}
