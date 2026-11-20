//! Unit tests for parse_lints.rs. The fixture is stdout from `clippy-driver -W help`, clippy 0.1.95, exit 0.

use super::parse_lints;
use procio::Output;

fn output(code: i32, stdout: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: Vec::new() }
}

fn fixture_lints() -> Vec<String> {
    parse_lints(&output(0, include_bytes!("fixtures/lint_groups_help.txt"))).unwrap()
}

#[test]
fn reads_rustc_and_clippy_lints_with_dashes_as_underscores() {
    let lints = fixture_lints();
    for name in ["clippy::bool_comparison", "unused_variables", "clippy::module_name_repetitions", "unknown_lints"] {
        assert!(lints.iter().any(|lint| lint == name), "{name} missing");
    }
}

#[test]
fn reads_no_group_and_no_rustdoc_lint() {
    let lints = fixture_lints();
    for name in ["clippy::style", "clippy::pedantic", "nonstandard_style"] {
        assert!(!lints.iter().any(|lint| lint == name), "{name} read as a lint");
    }
    assert!(!lints.iter().any(|lint| lint.starts_with("rustdoc::")));
}

#[test]
fn a_non_zero_exit_is_an_error() {
    assert!(parse_lints(&output(1, include_bytes!("fixtures/lint_groups_help.txt"))).is_err());
}

#[test]
fn help_without_a_lint_checks_section_is_an_error_quoting_it() {
    let got = parse_lints(&output(0, b"Available lint options:\n")).unwrap_err();
    assert_eq!(got.why, "clippy-driver -W help listed no lints: Available lint options:\n");
}
