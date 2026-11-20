//! Unit tests for parse_lint_groups.rs. The fixture is stdout from `clippy-driver -W help`, clippy 0.1.95, exit 0: 13 rustc
//! groups and 10 clippy groups.

use super::parse_lint_groups;
use procio::Output;

fn output(code: i32, stdout: &[u8]) -> Output {
    Output { code: Some(code), stdout: stdout.to_vec(), stderr: Vec::new() }
}

fn fixture_groups() -> Vec<String> {
    parse_lint_groups(&output(0, include_bytes!("fixtures/lint_groups_help.txt"))).unwrap()
}

#[test]
fn reads_rustc_and_clippy_groups_with_dashes_as_underscores() {
    let groups = fixture_groups();
    for name in ["warnings", "nonstandard_style", "unused", "clippy::style", "clippy::pedantic"] {
        assert!(groups.iter().any(|group| group == name), "{name} missing from {groups:?}");
    }
}

#[test]
fn reads_no_lint_from_the_lint_checks_sections() {
    let groups = fixture_groups();
    assert!(!groups.iter().any(|group| group == "unused_variables"));
    assert!(!groups.iter().any(|group| group == "clippy::bool_comparison"));
}

#[test]
fn reads_thirteen_rustc_and_ten_clippy_groups_in_order() {
    let groups = fixture_groups();
    assert_eq!(groups.len(), 23);
    assert_eq!(groups[0], "warnings");
    assert_eq!(groups[12], "unused");
    assert_eq!(groups[13], "clippy::all");
    assert_eq!(groups[22], "clippy::suspicious");
}

#[test]
fn a_non_zero_exit_is_an_error() {
    let got = parse_lint_groups(&output(1, include_bytes!("fixtures/lint_groups_help.txt")));
    assert!(got.is_err());
}

#[test]
fn help_without_a_groups_section_is_an_error_quoting_it() {
    let got = parse_lint_groups(&output(0, b"Available lint options:\n")).unwrap_err();
    assert_eq!(got.why, "clippy-driver -W help listed no lint groups: Available lint options:\n");
}

#[test]
fn a_groups_section_with_no_rows_is_an_error() {
    let help = b"Lint groups provided by rustc:\n\n    name  sub-lints\n    ----  ---------\n\n";
    assert!(parse_lint_groups(&output(0, help)).is_err());
}
