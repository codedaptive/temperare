//! ZUT for usage.rs: command, rule-file, no-short-name, no-link, savings and exit-code help as whole lines; the
//! loosened exits by their tokens.

use super::USAGE;

fn has_line(line: &str) -> bool {
    USAGE.lines().any(|l| l == line)
}

#[test]
fn has_the_check_line() {
    assert!(has_line("cctao check [--rules <path>] [--base <rev>] [--kind quality|security] [--json] [<path>...]"));
}

#[test]
fn has_the_rules_line() {
    assert!(has_line("cctao rules [--rules <path>] [--json]"));
}

#[test]
fn has_the_loosened_line_right_after_the_rules_line() {
    let lines: Vec<&str> = USAGE.lines().collect();
    let rules = lines.iter().position(|l| *l == "cctao rules [--rules <path>] [--json]");
    assert_eq!(
        rules.and_then(|i| lines.get(i + 1)).copied(),
        Some("cctao rules --loosened --base <rev> [--rules <path>] [--json]")
    );
}

#[test]
fn gives_the_loosened_exit_codes_after_the_check_exits() {
    let lines: Vec<&str> = USAGE.lines().collect();
    let precedence = lines.iter().position(|l| l.starts_with("The first that applies wins"));
    let line = precedence.and_then(|i| lines.get(i + 1)).copied().unwrap_or("");
    assert!(line.starts_with("cctao rules --loosened exits "), "{line}");
    for token in ["0 when nothing loosened", "1 when something did", "2 for a bad --base or rule file"] {
        assert!(line.contains(token), "{token}: {line}");
    }
}

#[test]
fn has_the_baseline_line() {
    assert!(has_line("cctao baseline test [--set <name@version>] [--json]"));
}

#[test]
fn has_the_help_and_version_lines() {
    assert!(has_line("cctao help"));
    assert!(has_line("cctao --version"));
}

#[test]
fn has_the_install_line() {
    assert!(has_line(
        "cctao install [--no-place] [--target claude,codex,cursor] [--mode skill] [--location local] [-y]"
    ));
    assert!(!USAGE.contains("--short-name"));
}

#[test]
fn has_the_uninstall_line() {
    assert!(has_line("cctao uninstall"));
}

#[test]
fn no_link_is_described() {
    assert!(!USAGE.contains("link"));
}

#[test]
fn says_where_the_rule_file_comes_from() {
    assert!(USAGE.lines().any(|line| line.contains("--rules <path>") && line.contains("tao.toml")));
}

#[test]
fn savings_contract_explains_the_savings_estimate() {
    assert!(has_line("When an engine ran for a rule other than a watch rule, cctao check's text output opens with an estimate of the tokens"));
    assert!(has_line("saved against the raw output of the engines it ran (characters / 4)."));
    assert!(!USAGE.lines().any(|line| line.starts_with("cctao check opens with")));
    assert!(has_line("It is an upper bound: the saving against reading that output whole."));
}

#[test]
fn savings_contract_explains_the_switch_and_json() {
    assert!(has_line("savings_line = false in ~/.config/cctao/config.toml turns the line and its ask off; the JSON saved object stays."));
}

#[test]
fn savings_contract_names_the_savings_log() {
    assert!(has_line("Each check that ran an engine for a rule other than a watch rule appends a line to"));
    assert!(has_line("~/.local/state/cctao/savings.jsonl."));
}

#[test]
fn lists_exit_6_and_the_exit_precedence() {
    assert!(has_line("6  a rule of the repository's own covers nothing"));
    assert!(has_line("The first that applies wins, in this order: 2, 1, 5, 3, 4, 6, 0."));
}

#[test]
fn ends_with_a_newline() {
    assert!(USAGE.ends_with('\n'));
}
