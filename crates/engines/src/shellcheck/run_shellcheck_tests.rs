//! Unit tests for the compiled-in list of shellcheck codes (`shellcheck_codes.txt`), the 510 codes ShellCheck 0.11.0's
//! source emits: how it reads, and which codes it lets through to the check run. The cases give the list to
//! `run_shellcheck_with` through `listed()` with a fake lister, as run_shellcheck.rs gives it with `shell_files`. No
//! engine or git runs. A malformed check is refused before this flow, by `shellcheck_refusal`, and is covered in
//! shellcheck_refusal_tests.rs.

use super::run_shellcheck_with_tests::{answers, listed, rule};
use crate::read_code_list::read_code_list;
use crate::types::Stop;
use std::path::PathBuf;

const CODES: &str = include_str!("shellcheck_codes.txt");

#[test]
fn the_compiled_in_list_reads_as_shellcheck_0_11_0s_codes() {
    let list = read_code_list(CODES).unwrap();
    assert_eq!(list.version, "0.11.0");
    assert!(list.source.contains("ShellCheck 0.11.0"), "{}", list.source);
    assert_eq!(list.codes.len(), 510);
    for code in ["SC1072", "SC2086", "SC2250", "SC3054"] {
        assert!(list.codes.contains(&code), "{code}");
    }
}

#[test]
fn a_listed_code_with_a_script_under_the_rules_paths_reaches_the_check_run() {
    let scripts = Ok(vec![PathBuf::from("scripts/run.sh")]);
    let (got, ran, raw_chars, _) = listed(CODES, scripts, &rule("SC2086", "paths = [\"scripts/**\"]\n"), &answers());
    assert!(got.is_ok(), "{got:?}");
    assert_eq!(ran.len(), 1, "{ran:?}");
    assert_eq!(ran[0].program, "shellcheck");
    assert_eq!(raw_chars, Some(862));
}

#[test]
fn a_well_formed_code_off_the_list_is_not_listed_and_nothing_runs() {
    let cases = [
        ("SC9999", "SC9999 is not in cctao's list of shellcheck codes (taken from shellcheck 0.11.0)"),
        ("SC02086", "SC02086 is not in cctao's list of shellcheck codes (taken from shellcheck 0.11.0)"),
    ];
    for (check, said) in cases {
        let (got, ran, raw_chars, _) = listed(CODES, Ok(vec![PathBuf::from("a.sh")]), &rule(check, ""), &answers());
        assert_eq!(got, Err(Stop::NotListed { said: said.to_string() }), "{check}");
        assert!(ran.is_empty(), "{check}: {ran:?}");
        assert_eq!(raw_chars, None, "{check}");
    }
}
