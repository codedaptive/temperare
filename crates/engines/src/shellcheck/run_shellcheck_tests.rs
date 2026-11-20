//! ZUT for run_shellcheck.rs on the compiled-in list, which is pending: it reads with no codes, so every well-formed
//! code is not available and nothing but the version probe runs; a malformed check is still refused. No engine runs.

use super::read_code_list;
use super::run_shellcheck::run_shellcheck;
use super::run_shellcheck_with_tests::{answers, entry, rule, run};
use crate::entry::Params;
use crate::types::Stop;
use std::ffi::OsString;

#[test]
fn the_compiled_in_list_reads_as_pending_with_no_codes() {
    let list = read_code_list(include_str!("shellcheck_codes.txt")).unwrap();
    assert_eq!(list.version, "pending");
    assert!(list.codes.is_empty(), "{:?}", list.codes);
}

#[test]
fn every_well_formed_code_is_not_available_while_the_list_is_pending() {
    let cases = [
        ("SC2086", "shellcheck 0.11.0: SC2086 is not in temperare's list of shellcheck codes (taken from shellcheck pending)"),
        ("SC2250", "shellcheck 0.11.0: SC2250 is not in temperare's list of shellcheck codes (taken from shellcheck pending)"),
        ("SC02086", "shellcheck 0.11.0: SC02086 is not in temperare's list of shellcheck codes (taken from shellcheck pending)"),
    ];
    for (check, why) in cases {
        let (got, ran, raw_chars) = run(run_shellcheck, &entry(Params::Args(&[])), &rule(check, ""), &answers());
        assert_eq!(got, Err(Stop::NotAvailable { why: why.to_string() }));
        assert_eq!(ran.len(), 1, "{ran:?}");
        assert_eq!(ran[0].args, vec![OsString::from("--version")]);
        assert_eq!(raw_chars, None);
    }
}

#[test]
fn a_malformed_check_is_still_refused() {
    let (got, ran, _) = run(run_shellcheck, &entry(Params::Args(&[])), &rule("SC", ""), &answers());
    assert_eq!(
        got,
        Err(Stop::Failed {
            code: None,
            stderr: "shellcheck check `SC` is not one ShellCheck code such as `SC2086`, so tm cannot tell which findings it selects"
                .to_string()
        })
    );
    assert!(ran.is_empty(), "{ran:?}");
}
