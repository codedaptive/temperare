//! ZUT for usage.rs: USAGE has each command line, the rule-file line and the line on when tm answers exactly, each
//! a whole line.

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
fn has_the_install_line() {
    assert!(has_line(
        "tm install [--no-place] [--short-name] [--target claude,codex,cursor] [--mode skill] [--location local] [-y]"
    ));
}

#[test]
fn has_the_uninstall_line() {
    assert!(has_line("tm uninstall"));
}

#[test]
fn says_tm_answers_only_where_install_made_the_link() {
    assert!(has_line(
        "temperare is the program; tm answers the same commands only where temperare install made the tm link."
    ));
    assert!(!USAGE.contains("both answer every command"));
}

#[test]
fn says_where_the_rule_file_comes_from() {
    assert!(has_line("The rule file is --rules <path>, else temperare.toml at the top of the git repository."));
}

#[test]
fn ends_with_a_newline() {
    assert!(USAGE.ends_with('\n'));
}
