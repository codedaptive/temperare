//! ZUT for shares_with.rs: each case that keeps two rules apart, by itself, and the cases that share.

use crate::shares_with::shares_with;
use rules::{Rule, RuleFile};
use std::path::Path;

/// One `[[rule]]` block of `engine` and `check`, with `extra` lines (config, params) after it.
fn block(id: &str, engine: &str, check: &str, extra: &str) -> String {
    format!("[[rule]]\nid = \"{id}\"\nengine = \"{engine}\"\ncheck = \"{check}\"\nfix = \"f\"\n{extra}")
}

/// Whether rule `a` shares with rule `b`, each given as (engine, check, extra lines), parsed from one rule file.
fn shares(a: (&str, &str, &str), b: (&str, &str, &str)) -> bool {
    let text = format!("version = 1\n{}{}", block("a", a.0, a.1, a.2), block("b", b.0, b.1, b.2));
    let file = RuleFile::parse(&text, Path::new(".")).unwrap();
    shares_with(&file.rules[0], &file.rules[1])
}

#[test]
fn different_engines_do_not_share() {
    assert!(!shares(("ruff", "E711", ""), ("swiftlint", "E711", "")));
}

#[test]
fn different_configs_do_not_share() {
    assert!(!shares(("swiftlint", "force_cast", "config = \"a.yml\"\n"), ("swiftlint", "force_try", "config = \"b.yml\"\n")));
    assert!(!shares(("swiftlint", "force_cast", "config = \"a.yml\"\n"), ("swiftlint", "force_try", "")));
}

#[test]
fn different_params_do_not_share() {
    assert!(!shares(("swiftlint", "force_cast", "params = { width = 80 }\n"), ("swiftlint", "force_try", "")));
}

#[test]
fn clippy_different_allow_does_not_share() {
    let a = ("clippy", "clippy::bool_comparison", "params = { allow = [\"clippy::needless_return\"] }\n");
    let b = ("clippy", "clippy::needless_bool", "");
    assert!(!shares(a, b));
}

#[test]
fn clippy_same_allow_shares() {
    let allow = "params = { allow = [\"clippy::needless_return\"] }\n";
    assert!(shares(("clippy", "clippy::bool_comparison", allow), ("clippy", "clippy::needless_bool", allow)));
}

#[test]
fn clippy_same_max_nesting_does_not_share() {
    let nesting = "params = { max_nesting = 3 }\n";
    assert!(!shares(("clippy", "clippy::excessive_nesting", nesting), ("clippy", "clippy::excessive_nesting", nesting)));
}

#[test]
fn swiftlint_same_max_lines_does_not_share() {
    let lines = "params = { max_lines = 400 }\n";
    assert!(!shares(("swiftlint", "file_length", lines), ("swiftlint", "file_length", lines)));
}

#[test]
fn ruff_same_max_complexity_shares() {
    let complexity = "params = { max_complexity = 10 }\n";
    assert!(shares(("ruff", "C901", complexity), ("ruff", "E711", complexity)));
}

#[test]
fn clippy_warnings_does_not_share_either_way() {
    assert!(!shares(("clippy", "warnings", ""), ("clippy", "clippy::needless_bool", "")));
    assert!(!shares(("clippy", "clippy::needless_bool", ""), ("clippy", "warnings", "")));
}

#[test]
fn clippy_equal_warnings_checks_share() {
    assert!(shares(("clippy", "warnings", ""), ("clippy", "warnings", "")));
}

#[test]
fn clippy_two_lints_share() {
    assert!(shares(("clippy", "clippy::bool_comparison", ""), ("clippy", "clippy::needless_bool", "")));
}

#[test]
fn ruff_all_does_not_share_either_way() {
    assert!(!shares(("ruff", "ALL", ""), ("ruff", "E711", "")));
    assert!(!shares(("ruff", "E711", ""), ("ruff", "F401,ALL", "")));
}

#[test]
fn ruff_prefix_overlap_does_not_share_either_way() {
    assert!(!shares(("ruff", "E7", ""), ("ruff", "E711", "")));
    assert!(!shares(("ruff", "E711", ""), ("ruff", "E7", "")));
}

#[test]
fn ruff_prefix_overlap_inside_a_comma_list_does_not_share() {
    assert!(!shares(("ruff", "F401,E7", ""), ("ruff", "W291,E711", "")));
}

#[test]
fn ruff_equal_checks_share() {
    assert!(shares(("ruff", "E711", ""), ("ruff", "E711", "")));
    assert!(shares(("ruff", "ALL", ""), ("ruff", "ALL", "")));
}

#[test]
fn ruff_disjoint_checks_share() {
    assert!(shares(("ruff", "E711", ""), ("ruff", "F401", "")));
    assert!(shares(("ruff", "E711,E712", ""), ("ruff", "F401", "")));
}

#[test]
fn ruff_check_selecting_ruf100_does_not_share_either_way() {
    assert!(!shares(("ruff", "RUF100", ""), ("ruff", "E711", "")));
    assert!(!shares(("ruff", "E711", ""), ("ruff", "RUF100", "")));
    assert!(!shares(("ruff", "RUF", ""), ("ruff", "E711", "")));
}

#[test]
fn ruff_equal_ruf100_checks_share() {
    assert!(shares(("ruff", "RUF100", ""), ("ruff", "RUF100", "")));
}

#[test]
fn swiftlint_superfluous_disable_command_does_not_share_either_way() {
    assert!(!shares(("swiftlint", "superfluous_disable_command", ""), ("swiftlint", "force_cast", "")));
    assert!(!shares(("swiftlint", "force_cast", ""), ("swiftlint", "superfluous_disable_command", "")));
}

#[test]
fn swiftlint_superfluous_disable_command_shares_with_itself() {
    assert!(shares(("swiftlint", "superfluous_disable_command", ""), ("swiftlint", "superfluous_disable_command", "")));
}

#[test]
fn swiftlint_two_ordinary_rules_share() {
    assert!(shares(("swiftlint", "force_cast", ""), ("swiftlint", "force_try", "")));
    assert!(shares(("swiftlint", "force_cast", "config = \"a.yml\"\n"), ("swiftlint", "force_cast", "config = \"a.yml\"\n")));
}

#[test]
fn ast_grep_same_yaml_without_params_shares() {
    assert!(shares(("ast-grep", "rules/no-unwrap.yml", ""), ("ast-grep", "rules/no-unwrap.yml", "paths = [\"src/**\"]\n")));
}

#[test]
fn ast_grep_different_yamls_do_not_share() {
    assert!(!shares(("ast-grep", "rules/no-unwrap.yml", ""), ("ast-grep", "rules/no-expect.yml", "")));
}

#[test]
fn ast_grep_same_yaml_with_params_does_not_share() {
    let names = "params = { names = [\"foo\"] }\n";
    assert!(!shares(("ast-grep", "rules/banned.yml", names), ("ast-grep", "rules/banned.yml", names)));
}

#[test]
fn tessera_never_shares() {
    assert!(!shares(("tessera", "slop", ""), ("tessera", "slop", "")));
    assert!(!shares(("tessera", "slop", ""), ("tessera", "dead", "")));
}

#[test]
fn ruff_d203_and_d211_do_not_share_either_way() {
    assert!(!shares(("ruff", "D203", ""), ("ruff", "D211", "")));
    assert!(!shares(("ruff", "D211", ""), ("ruff", "D203", "")));
}

#[test]
fn ruff_d212_and_d213_do_not_share_either_way() {
    assert!(!shares(("ruff", "D212", ""), ("ruff", "D213", "")));
    assert!(!shares(("ruff", "D213", ""), ("ruff", "D212", "")));
}

#[test]
fn ruff_split_pair_inside_a_comma_list_does_not_share() {
    assert!(!shares(("ruff", "E711,D203", ""), ("ruff", "F401,D211", "")));
}

#[test]
fn ruff_check_selecting_a_whole_pair_shares_with_a_disjoint_check() {
    assert!(shares(("ruff", "D", ""), ("ruff", "E711", "")));
    assert!(shares(("ruff", "E711", ""), ("ruff", "D", "")));
}

#[test]
fn ruff_halves_of_different_pairs_share() {
    assert!(shares(("ruff", "D203", ""), ("ruff", "D212", "")));
}

#[test]
fn ruff_equal_pair_halves_share() {
    assert!(shares(("ruff", "D203", ""), ("ruff", "D203", "")));
}

#[test]
fn shellcheck_never_shares_even_with_the_same_check_config_and_params() {
    assert!(!shares(("shellcheck", "SC2086", ""), ("shellcheck", "SC2086", "")));
    assert!(!shares(("shellcheck", "SC2086", "config = \"x.rc\"\n"), ("shellcheck", "SC2086", "config = \"x.rc\"\n")));
    assert!(!shares(("shellcheck", "SC2086", ""), ("shellcheck", "SC2250", "")));
}

#[test]
fn git_never_shares_even_with_the_same_check_and_params() {
    let params = "params = { max_ratio = 1.5, floor = 40, test_paths = [\"tests/**\"] }\n";
    assert!(!shares(("git", "test_ratio", params), ("git", "test_ratio", params)));
}

#[test]
fn an_engine_with_no_adapter_never_shares() {
    assert!(!shares(("oclint", "LongLine", ""), ("oclint", "LongLine", "")));
}

/// Two ast-grep rules of check `rules/no-unwrap.yml` with no params, carrying `a` and `b` as their YAML text.
fn ast_grep_with_texts(a: Option<&'static str>, b: Option<&'static str>) -> (Rule, Rule) {
    let text = format!("version = 1\n{}{}", block("a", "ast-grep", "rules/no-unwrap.yml", ""), block("b", "ast-grep", "rules/no-unwrap.yml", ""));
    let mut rules = RuleFile::parse(&text, Path::new(".")).unwrap().rules;
    let mut second = rules.remove(1);
    let mut first = rules.remove(0);
    first.embedded_text = a;
    second.embedded_text = b;
    (first, second)
}

#[test]
fn ast_grep_repository_rule_and_set_rule_with_the_same_check_do_not_share_either_way() {
    let (repo, set) = ast_grep_with_texts(None, Some("id: x\nlanguage: rust\nrule:\n  pattern: $A.unwrap()\n"));
    assert!(!shares_with(&repo, &set));
    assert!(!shares_with(&set, &repo));
}

#[test]
fn ast_grep_set_rules_with_the_same_check_and_text_share() {
    let yaml = "id: x\nlanguage: rust\nrule:\n  pattern: $A.unwrap()\n";
    let (a, b) = ast_grep_with_texts(Some(yaml), Some(yaml));
    assert!(shares_with(&a, &b));
}

#[test]
fn ast_grep_set_rules_with_the_same_check_and_different_texts_do_not_share() {
    let (a, b) = ast_grep_with_texts(
        Some("id: x\nlanguage: rust\nrule:\n  pattern: $A.unwrap()\n"),
        Some("id: x\nlanguage: rust\nrule:\n  pattern: $A.expect($B)\n"),
    );
    assert!(!shares_with(&a, &b));
}
