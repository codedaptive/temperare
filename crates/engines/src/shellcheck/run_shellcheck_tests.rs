//! ZUT for the compiled-in list of shellcheck codes (`shellcheck_codes.txt`), which is pending: it reads with no codes,
//! so a rule with a script to check is not listed and the check run never happens. Those cases give the list to
//! `run_shellcheck_with` through `listed()` with a fake lister, as run_shellcheck.rs gives it with `shell_files`. No
//! engine or git runs.

use super::read_code_list;
use super::run_shellcheck_with_tests::{answers, listed, rule};
use crate::types::Stop;
use std::path::PathBuf;

const CODES: &str = include_str!("shellcheck_codes.txt");

#[test]
fn the_compiled_in_list_reads_as_pending_with_no_codes() {
    let list = read_code_list(CODES).unwrap();
    assert_eq!(list.version, "pending");
    assert!(list.codes.is_empty(), "{:?}", list.codes);
}

#[test]
fn every_well_formed_code_is_not_listed_while_the_list_is_pending() {
    let cases = [
        ("SC2086", "SC2086 is not in temperare's list of shellcheck codes (taken from shellcheck pending)"),
        ("SC2250", "SC2250 is not in temperare's list of shellcheck codes (taken from shellcheck pending)"),
        ("SC02086", "SC02086 is not in temperare's list of shellcheck codes (taken from shellcheck pending)"),
    ];
    for (check, said) in cases {
        let (got, ran, raw_chars, _) = listed(CODES, Ok(vec![PathBuf::from("a.sh")]), &rule(check, ""), &answers());
        assert_eq!(got, Err(Stop::NotListed { said: said.to_string() }));
        assert!(ran.is_empty(), "{ran:?}");
        assert_eq!(raw_chars, None);
    }
}

#[test]
fn a_script_under_the_rules_paths_is_never_checked_while_the_list_is_pending() {
    let scripts = Ok(vec![PathBuf::from("scripts/run.sh")]);
    let (got, ran, raw_chars, _) = listed(CODES, scripts, &rule("SC2086", "paths = [\"scripts/**\"]\n"), &answers());
    let said = "SC2086 is not in temperare's list of shellcheck codes (taken from shellcheck pending)";
    assert_eq!(got, Err(Stop::NotListed { said: said.to_string() }));
    assert!(ran.is_empty(), "{ran:?}");
    assert_eq!(raw_chars, None);
}
