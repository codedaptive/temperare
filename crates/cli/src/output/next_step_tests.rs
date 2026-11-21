//! ZUT for next_step.rs: the next-step text for every exit code, each expected line a literal.

use super::next_step;
use engines::NotAvailableCause::{self, EngineNotOnPath, HelperNotOnPath, Other, UnknownCheck};
use report::NotAvailable;
use rules::Engine;
use std::path::Path;

const CARGO_WHY: &str = "no Cargo.toml at or above the rule file's folder";
const UNKNOWN_WHY: &str =
    "ruff 0.6.9 does not know the check `F999`: Unknown rule selector `F999` in `select` from the CLI";

type Entry = (NotAvailable, Option<NotAvailableCause>);

fn file() -> &'static Path {
    Path::new("repo/temperare.toml")
}

/// `next_step` given `entries`, each entry paired with its cause as run_check pairs them.
fn step(exit: u8, fix: Option<&str>, entries: &[Entry]) -> String {
    let pairs: Vec<_> = entries.iter().map(|(entry, cause)| (entry, *cause)).collect();
    next_step(exit, fix, file(), &pairs, false)
}

fn entry(engine: Engine, why: &str, cause: Option<NotAvailableCause>) -> Entry {
    (NotAvailable { rule: "r".to_string(), engine, why: why.to_string() }, cause)
}

/// An engine whose program is missing: the why engines gives for it (`<program> not found on PATH`) with the cause
/// `EngineNotOnPath`; the program is `Engine::name` except for Clippy (`cargo-clippy`) and Tessera (`ts`; engines
/// never reports Tessera this way, so its why is made up here).
fn absent(engine: Engine) -> Entry {
    let program = match engine {
        Engine::Clippy => "cargo-clippy",
        Engine::Tessera => "ts",
        other => other.name(),
    };
    entry(engine, &format!("{program} not found on PATH"), Some(EngineNotOnPath))
}

#[test]
fn exit_0_has_nothing_to_do_whatever_the_other_arguments() {
    assert_eq!(step(0, None, &[]), "nothing to do");
    assert_eq!(step(0, Some("Write it."), &[absent(Engine::Ruff)]), "nothing to do");
}

#[test]
fn exit_1_fixes_the_findings_or_adds_an_exception_to_the_rule_file() {
    assert_eq!(
        step(1, Some("Write it."), &[absent(Engine::Ruff)]),
        "fix the findings above, or add an approved [[exception]] to repo/temperare.toml"
    );
}

#[test]
fn exit_2_gives_the_fix() {
    assert_eq!(
        step(2, Some("Run tm help to see the commands."), &[absent(Engine::Ruff)]),
        "Run tm help to see the commands."
    );
}

#[test]
fn exit_2_without_a_fix_points_at_the_error() {
    assert_eq!(step(2, None, &[]), "see the error above");
}

#[test]
fn exit_3_names_the_engine_to_install() {
    let got = step(3, Some("Write it."), &[absent(Engine::Ruff)]);
    assert_eq!(got, "install ruff (or run without those rules)");
}

#[test]
fn exit_3_names_two_engines_in_the_order_given() {
    assert_eq!(
        step(3, None, &[absent(Engine::SwiftLint), absent(Engine::AstGrep)]),
        "install swiftlint, ast-grep (or run without those rules)"
    );
}

#[test]
fn exit_3_with_no_entries_speaks_of_the_engines_not_available() {
    assert_eq!(
        step(3, None, &[]),
        "install the engines that are not available (or run without those rules)"
    );
}

#[test]
fn exit_3_with_only_tessera_says_its_checks_are_not_available_yet() {
    assert_eq!(
        step(3, None, &[entry(Engine::Tessera, "Tessera's slop records are not defined yet", Some(Other))]),
        "Tessera's checks are not available yet (run without those rules)"
    );
}

#[test]
fn exit_3_with_tessera_first_installs_the_others_and_says_tessera_is_not_available_yet() {
    assert_eq!(
        step(3, None, &[absent(Engine::Tessera), absent(Engine::Ruff)]),
        "install ruff; Tessera's checks are not available yet (or run without those rules)"
    );
}

#[test]
fn exit_3_with_tessera_in_the_middle_keeps_the_others_in_the_order_given() {
    assert_eq!(
        step(3, None, &[absent(Engine::Ruff), absent(Engine::Tessera), absent(Engine::Clippy)]),
        "install ruff, clippy; Tessera's checks are not available yet (or run without those rules)"
    );
}

#[test]
fn next_step_missing_engine_and_unknown_check_combine_into_one_next_step() {
    assert_eq!(
        step(3, None, &[entry(Engine::Ruff, UNKNOWN_WHY, Some(UnknownCheck)), absent(Engine::SwiftLint)]),
        "install swiftlint; ruff: ruff 0.6.9 does not know the check `F999`: Unknown rule selector `F999` in `select` from the CLI (or run without those rules)"
    );
}

#[test]
fn exit_3_names_the_same_cause_once() {
    assert_eq!(
        step(3, None, &[entry(Engine::Clippy, CARGO_WHY, Some(Other)), entry(Engine::Clippy, CARGO_WHY, Some(Other))]),
        "clippy: no Cargo.toml at or above the rule file's folder, so put the rule file inside the Cargo project it checks (or run without those rules)"
    );
}

#[test]
fn exit_3_puts_install_then_causes_then_tessera() {
    let entries = [absent(Engine::Tessera), entry(Engine::Clippy, "boom", Some(Other)), absent(Engine::Ruff), absent(Engine::Ruff)];
    assert_eq!(
        step(3, None, &entries),
        "install ruff; clippy: boom; Tessera's checks are not available yet (or run without those rules)"
    );
}

#[test]
fn exit_3_installs_only_for_an_engine_not_on_path() {
    let rows = [
        (Engine::Clippy, "clippy-driver not found on PATH", Some(EngineNotOnPath), "install clippy (or run without those rules)"),
        (Engine::Ruff, "the engine's own program is missing", Some(EngineNotOnPath), "install ruff (or run without those rules)"),
        (Engine::Clippy, "cargo not found on PATH", Some(HelperNotOnPath), "clippy: cargo not found on PATH (or run without those rules)"),
        (
            Engine::ShellCheck,
            "git not found on PATH",
            Some(HelperNotOnPath),
            "shellcheck: git not found on PATH (or run without those rules)",
        ),
        (Engine::Ruff, "ruff not found on PATH", Some(HelperNotOnPath), "ruff: ruff not found on PATH (or run without those rules)"),
        (Engine::Ruff, "ruff not found on PATH", None, "ruff: ruff not found on PATH (or run without those rules)"),
        (
            Engine::Ruff,
            UNKNOWN_WHY,
            Some(UnknownCheck),
            "ruff: ruff 0.6.9 does not know the check `F999`: Unknown rule selector `F999` in `select` from the CLI (or run without those rules)",
        ),
        (
            Engine::Clippy,
            CARGO_WHY,
            Some(Other),
            "clippy: no Cargo.toml at or above the rule file's folder, so put the rule file inside the Cargo project it checks (or run without those rules)",
        ),
        (Engine::Clippy, "boom", Some(Other), "clippy: boom (or run without those rules)"),
    ];
    let wrong: Vec<String> = rows
        .into_iter()
        .filter_map(|(engine, why, cause, want)| {
            let got = step(3, None, &[entry(engine, why, cause)]);
            (got != want).then(|| format!("{engine:?} {why} {cause:?}:\n  got:  {got}\n  want: {want}"))
        })
        .collect();
    assert!(wrong.is_empty(), "rows wrong:\n{}", wrong.join("\n"));
}

#[test]
fn exit_4_raises_the_security_findings() {
    assert_eq!(
        step(4, Some("Write it."), &[absent(Engine::Ruff)]),
        "raise the security findings with their owner; work goes on"
    );
}

#[test]
fn exit_5_and_unknown_codes_point_at_the_output() {
    assert_eq!(step(5, Some("Write it."), &[absent(Engine::Ruff)]), "see the engine output above");
    assert_eq!(step(7, Some("Write it."), &[absent(Engine::Ruff)]), "see the output above");
    assert_eq!(step(255, None, &[]), "see the output above");
}

#[test]
fn next_step_exit_6_names_the_rule_file() {
    let (entry, cause) = absent(Engine::Ruff);
    assert_eq!(
        next_step(6, Some("Write it."), Path::new("team/rules/temperare.toml"), &[(&entry, cause)], false),
        "fix the paths of each rule that covers nothing above, or remove it from team/rules/temperare.toml"
    );
}

#[test]
fn next_step_exit_6_under_json_names_covers_nothing_and_the_rule_file() {
    let (entry, cause) = absent(Engine::Ruff);
    assert_eq!(
        next_step(6, Some("Write it."), Path::new("team/rules/temperare.toml"), &[(&entry, cause)], true),
        "fix the paths of each rule in covers_nothing, or remove it from team/rules/temperare.toml"
    );
}
