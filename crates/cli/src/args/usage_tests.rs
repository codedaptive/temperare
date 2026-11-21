//! ZUT for usage.rs: command, rule-file, short-name and savings help as whole lines.

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
fn has_the_baseline_line() {
    assert!(has_line("tm baseline test [--set <name@version>] [--json]"));
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
fn savings_contract_explains_the_savings_estimate() {
    assert!(has_line("When at least one engine ran, tm check's text output opens with an estimate of the tokens saved against the raw"));
    assert!(has_line("output of the engines it ran (characters / 4)."));
    assert!(!USAGE.lines().any(|line| line.starts_with("tm check opens with")));
    assert!(has_line("It is an upper bound: the saving against reading that output whole."));
}

#[test]
fn savings_contract_explains_the_switch_and_json() {
    assert!(has_line("savings_line = false in ~/.config/temperare/config.toml turns the line and its ask off; the JSON saved object stays."));
}

#[test]
fn savings_contract_names_the_savings_log() {
    assert!(has_line("Each check that ran an engine appends a line to ~/.local/state/temperare/savings.jsonl."));
}

#[test]
fn ends_with_a_newline() {
    assert!(USAGE.ends_with('\n'));
}
