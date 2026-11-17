//! ZUT for usage.rs: USAGE has each command line and the rule-file line exactly, each a whole line.

use super::USAGE;

fn has_line(line: &str) -> bool {
    USAGE.lines().any(|l| l == line)
}

#[test]
fn has_the_check_line() {
    assert!(has_line("tm check [--rules <path>] [--base <rev>] [--kind quality|security] [--json] [<path>...]"));
}

#[test]
fn has_the_rules_line() {
    assert!(has_line("tm rules [--rules <path>] [--json]"));
}

#[test]
fn has_the_help_and_version_lines() {
    assert!(has_line("tm help"));
    assert!(has_line("tm --version"));
}

#[test]
fn says_where_the_rule_file_comes_from() {
    assert!(has_line("The rule file is --rules <path>, else temperare.toml at the top of the git repository."));
}

#[test]
fn ends_with_a_newline() {
    assert!(USAGE.ends_with('\n'));
}
