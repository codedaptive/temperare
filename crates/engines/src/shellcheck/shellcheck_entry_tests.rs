//! ZUT for shellcheck_entry.rs: the fields of `SHELLCHECK`, its version probe read on real `shellcheck --version`
//! output (fixtures/version.txt, 0.11.0), its refusal of a malformed code and its unknown-check reader. Nothing runs.

use super::SHELLCHECK;
use crate::entry::{Params, Pick};
use procio::Output;
use rules::Engine;

fn out(code: i32, stdout: &[u8], stderr: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: stderr.to_vec() }
}

#[test]
fn the_entry_is_shellcheck_needing_shellcheck() {
    assert_eq!((SHELLCHECK.name, SHELLCHECK.engine, SHELLCHECK.program), ("shellcheck", Engine::ShellCheck, "shellcheck"));
    assert_eq!(SHELLCHECK.needs, ["shellcheck"]);
    assert_eq!(SHELLCHECK.never, None);
}

#[test]
fn the_check_is_never_passed_and_the_config_is_the_rcfile() {
    assert_eq!(SHELLCHECK.pick, Pick::None);
    assert_eq!(SHELLCHECK.config, Some("--rcfile="));
    assert!(SHELLCHECK.args.is_empty() && SHELLCHECK.after.is_empty());
}

#[test]
fn it_takes_no_params() {
    assert!(matches!(SHELLCHECK.params, Params::Args(args) if args.is_empty()), "{:?}", SHELLCHECK.params);
}

#[test]
fn the_version_is_read_from_shellcheck_dash_dash_version() {
    let probe = SHELLCHECK.version.unwrap();
    assert_eq!(probe.args, ["--version"]);
    assert_eq!((probe.read)(&out(0, include_bytes!("fixtures/version.txt"), b"")), Some("0.11.0".to_string()));
}

#[test]
fn a_lowercase_code_is_refused_and_a_shellcheck_code_is_not() {
    assert_eq!(
        (SHELLCHECK.refuse)("sc2086"),
        Some("shellcheck check `sc2086` is not one ShellCheck code such as `SC2086`, so tm cannot tell which findings it selects".to_string())
    );
    assert_eq!((SHELLCHECK.refuse)("SC2086"), None);
}

#[test]
fn shellcheck_never_says_it_does_not_know_a_code() {
    let outputs = [
        out(0, b"", b""),
        out(1, include_bytes!("fixtures/sc2086.json"), b""),
        out(3, b"", include_bytes!("fixtures/bad_option.stderr")),
        out(2, b"", b"SC9999 unknown"),
    ];
    for output in &outputs {
        assert_eq!((SHELLCHECK.unknown)(output, "SC9999"), None, "{output:?}");
        assert_eq!((SHELLCHECK.unknown)(output, "SC2086"), None, "{output:?}");
    }
}
