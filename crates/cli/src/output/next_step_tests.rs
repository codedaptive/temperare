//! Unit tests for next_step.rs: the next-step text for every exit code, each expected line a literal.

use super::next_step;
use report::NotAvailable;
use rules::Engine;
use std::path::Path;

const CARGO_WHY: &str = "no Cargo.toml at or above the rule file's folder";

fn file() -> &'static Path {
    Path::new("repo/temperare.toml")
}

fn entry(engine: Engine, why: &str) -> NotAvailable {
    NotAvailable { rule: "r".to_string(), engine, why: why.to_string() }
}

/// An engine whose program is missing, with the why engines gives for it.
fn absent(engine: Engine) -> NotAvailable {
    let program = match engine {
        Engine::Clippy => "cargo-clippy",
        Engine::SwiftLint => "swiftlint",
        Engine::Ruff => "ruff",
        Engine::AstGrep => "ast-grep",
        Engine::Tessera => "ts",
    };
    entry(engine, &format!("{program} not found on PATH"))
}

#[test]
fn exit_0_has_nothing_to_do_whatever_the_other_arguments() {
    assert_eq!(next_step(0, None, file(), &[]), "nothing to do");
    assert_eq!(next_step(0, Some("Write it."), file(), &[absent(Engine::Ruff)]), "nothing to do");
}

#[test]
fn exit_1_fixes_the_findings_or_adds_an_exception_to_the_rule_file() {
    assert_eq!(
        next_step(1, Some("Write it."), file(), &[absent(Engine::Ruff)]),
        "fix the findings above, or add an approved [[exception]] to repo/temperare.toml"
    );
}

#[test]
fn exit_2_gives_the_fix() {
    assert_eq!(
        next_step(2, Some("Run tm help to see the commands."), file(), &[absent(Engine::Ruff)]),
        "Run tm help to see the commands."
    );
}

#[test]
fn exit_2_without_a_fix_points_at_the_error() {
    assert_eq!(next_step(2, None, file(), &[]), "see the error above");
}

#[test]
fn exit_3_names_the_engine_to_install() {
    let got = next_step(3, Some("Write it."), file(), &[absent(Engine::Ruff)]);
    assert_eq!(got, "install ruff (or run without those rules)");
}

#[test]
fn exit_3_names_two_engines_in_the_order_given() {
    assert_eq!(
        next_step(3, None, file(), &[absent(Engine::SwiftLint), absent(Engine::AstGrep)]),
        "install swiftlint, ast-grep (or run without those rules)"
    );
}

#[test]
fn exit_3_with_no_entries_speaks_of_the_engines_not_available() {
    assert_eq!(
        next_step(3, None, file(), &[]),
        "install the engines that are not available (or run without those rules)"
    );
}

#[test]
fn exit_3_with_only_tessera_says_its_checks_are_not_available_yet() {
    assert_eq!(
        next_step(3, None, file(), &[entry(Engine::Tessera, "Tessera's slop records are not defined yet")]),
        "Tessera's checks are not available yet (run without those rules)"
    );
}

#[test]
fn exit_3_with_tessera_first_installs_the_others_and_says_tessera_is_not_available_yet() {
    assert_eq!(
        next_step(3, None, file(), &[absent(Engine::Tessera), absent(Engine::Ruff)]),
        "install ruff; Tessera's checks are not available yet (or run without those rules)"
    );
}

#[test]
fn exit_3_with_tessera_in_the_middle_keeps_the_others_in_the_order_given() {
    assert_eq!(
        next_step(3, None, file(), &[absent(Engine::Ruff), absent(Engine::Tessera), absent(Engine::Clippy)]),
        "install ruff, clippy; Tessera's checks are not available yet (or run without those rules)"
    );
}

#[test]
fn exit_3_with_no_cargo_project_says_where_the_rule_file_goes() {
    assert_eq!(
        next_step(3, None, file(), &[entry(Engine::Clippy, CARGO_WHY)]),
        "clippy: no Cargo.toml at or above the rule file's folder, so put the rule file inside the Cargo project it checks (or run without those rules)"
    );
}

#[test]
fn exit_3_with_an_unknown_why_gives_the_engine_and_its_why() {
    let got = next_step(3, None, file(), &[entry(Engine::Clippy, "boom")]);
    assert_eq!(got, "clippy: boom (or run without those rules)");
}

#[test]
fn exit_3_names_the_same_cause_once() {
    assert_eq!(
        next_step(3, None, file(), &[entry(Engine::Clippy, CARGO_WHY), entry(Engine::Clippy, CARGO_WHY)]),
        "clippy: no Cargo.toml at or above the rule file's folder, so put the rule file inside the Cargo project it checks (or run without those rules)"
    );
}

#[test]
fn exit_3_puts_install_then_causes_then_tessera() {
    let entries = [absent(Engine::Tessera), entry(Engine::Clippy, "boom"), absent(Engine::Ruff), absent(Engine::Ruff)];
    assert_eq!(
        next_step(3, None, file(), &entries),
        "install ruff; clippy: boom; Tessera's checks are not available yet (or run without those rules)"
    );
}

#[test]
fn exit_4_raises_the_security_findings() {
    assert_eq!(
        next_step(4, Some("Write it."), file(), &[absent(Engine::Ruff)]),
        "raise the security findings with their owner; work goes on"
    );
}

#[test]
fn exit_5_and_unknown_codes_point_at_the_output() {
    assert_eq!(next_step(5, Some("Write it."), file(), &[absent(Engine::Ruff)]), "see the engine output above");
    assert_eq!(next_step(7, Some("Write it."), file(), &[absent(Engine::Ruff)]), "see the output above");
    assert_eq!(next_step(255, None, file(), &[]), "see the output above");
}
