//! ZUT for next_step.rs: the next-step text for every exit code, each expected line a literal.

use super::next_step;
use std::path::Path;

fn file() -> &'static Path {
    Path::new("repo/temperare.toml")
}

#[test]
fn exit_0_has_nothing_to_do_whatever_the_other_arguments() {
    assert_eq!(next_step(0, None, file(), &[]), "nothing to do");
    assert_eq!(next_step(0, Some("Write it."), file(), &["ruff"]), "nothing to do");
}

#[test]
fn exit_1_fixes_the_findings_or_adds_an_exception_to_the_rule_file() {
    assert_eq!(
        next_step(1, Some("Write it."), file(), &["ruff"]),
        "fix the findings above, or add an approved [[exception]] to repo/temperare.toml"
    );
}

#[test]
fn exit_2_gives_the_fix() {
    assert_eq!(
        next_step(2, Some("Run tm help to see the commands."), file(), &["ruff"]),
        "Run tm help to see the commands."
    );
}

#[test]
fn exit_2_without_a_fix_points_at_the_error() {
    assert_eq!(next_step(2, None, file(), &[]), "see the error above");
}

#[test]
fn exit_3_names_the_engine_to_install() {
    assert_eq!(next_step(3, Some("Write it."), file(), &["ruff"]), "install ruff (or run without those rules)");
}

#[test]
fn exit_3_names_two_engines_in_the_order_given() {
    assert_eq!(
        next_step(3, None, file(), &["swiftlint", "ast-grep"]),
        "install swiftlint, ast-grep (or run without those rules)"
    );
}

#[test]
fn exit_3_with_no_names_speaks_of_the_engines_not_available() {
    assert_eq!(
        next_step(3, None, file(), &[]),
        "install the engines that are not available (or run without those rules)"
    );
}

#[test]
fn exit_3_with_only_tessera_says_its_checks_are_not_available_yet() {
    assert_eq!(
        next_step(3, None, file(), &["tessera"]),
        "Tessera's checks are not available yet (run without those rules)"
    );
}

#[test]
fn exit_3_with_tessera_first_installs_the_others_and_says_tessera_is_not_available_yet() {
    assert_eq!(
        next_step(3, None, file(), &["tessera", "ruff"]),
        "install ruff; Tessera's checks are not available yet (or run without those rules)"
    );
}

#[test]
fn exit_3_with_tessera_in_the_middle_keeps_the_others_in_the_order_given() {
    assert_eq!(
        next_step(3, None, file(), &["ruff", "tessera", "clippy"]),
        "install ruff, clippy; Tessera's checks are not available yet (or run without those rules)"
    );
}

#[test]
fn exit_4_raises_the_security_findings() {
    assert_eq!(
        next_step(4, Some("Write it."), file(), &["ruff"]),
        "raise the security findings with their owner; work goes on"
    );
}

#[test]
fn exit_5_points_at_the_engine_output() {
    assert_eq!(next_step(5, Some("Write it."), file(), &["ruff"]), "see the engine output above");
}

#[test]
fn an_unknown_code_points_at_the_output() {
    assert_eq!(next_step(7, Some("Write it."), file(), &["ruff"]), "see the output above");
    assert_eq!(next_step(255, None, file(), &[]), "see the output above");
}
