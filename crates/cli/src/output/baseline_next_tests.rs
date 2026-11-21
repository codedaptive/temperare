//! ZUT for baseline_next.rs: the next-step text for each exit code, each expected line a literal.

use super::baseline_next;
use super::baseline_rule_result::BaselineRuleResult;
use super::case_verdict::CaseVerdict;
use rules::Engine;

fn result(rule: &str, engine: Engine, failing: CaseVerdict) -> BaselineRuleResult {
    BaselineRuleResult {
        set: "standard@1".to_string(),
        rule: rule.to_string(),
        engine,
        failing,
        passing: CaseVerdict::Holds,
    }
}

/// A rule whose engine is absent: the why engines give when its program is not on PATH (`<program> not found on
/// PATH`); the program is `Engine::name` except for Clippy (`cargo-clippy`).
fn not_available(rule: &str, engine: Engine) -> BaselineRuleResult {
    let program = match engine {
        Engine::Clippy => "cargo-clippy",
        other => other.name(),
    };
    not_available_why(rule, engine, &format!("{program} not found on PATH"))
}

/// A rule whose failing case is not available with `why`.
fn not_available_why(rule: &str, engine: Engine, why: &str) -> BaselineRuleResult {
    result(rule, engine, CaseVerdict::NotAvailable { why: why.to_string() })
}

const UNKNOWN_CHECK: &str =
    "ruff 0.4.1 does not know the check `XYZ123`: error: invalid value 'XYZ123' for '--select'";

/// A ruff rule whose installed ruff does not know its check: both cases carry an unknown-check why of the shape `too_old_why` gives.
fn unknown_check(rule: &str) -> BaselineRuleResult {
    BaselineRuleResult {
        passing: CaseVerdict::NotAvailable { why: UNKNOWN_CHECK.to_string() },
        ..result(rule, Engine::Ruff, CaseVerdict::NotAvailable { why: UNKNOWN_CHECK.to_string() })
    }
}

fn not_applicable(rule: &str) -> BaselineRuleResult {
    let why = "required params not given: entity_types".to_string();
    result(rule, Engine::Ruff, CaseVerdict::NotApplicable { why })
}

#[test]
fn baseline_next_text() {
    let rows = [
        ("nothing to do", 0, vec![result("tm-zeta", Engine::Ruff, CaseVerdict::Holds)], "nothing to do"),
        (
            "broken",
            1,
            vec![result("tm-zeta", Engine::Ruff, CaseVerdict::Broken { why: "gave no finding".to_string() })],
            "fix each broken rule or its cases above",
        ),
        (
            "each engine once, in first-named order, skipping rules that hold",
            3,
            vec![
                not_available("tm-zeta", Engine::SwiftLint),
                result("tm-mid", Engine::Clippy, CaseVerdict::Holds),
                not_available("tm-alpha", Engine::Ruff),
                not_available("tm-beta", Engine::SwiftLint),
            ],
            "install swiftlint, ruff, then run tm baseline test again",
        ),
        (
            "not applicable only",
            3,
            vec![result("tm-zeta", Engine::Ruff, CaseVerdict::Holds), not_applicable("tm-alpha")],
            "each not-applicable rule above says why it was not run",
        ),
        (
            "not applicable and not available: both parts, install first",
            3,
            vec![not_applicable("tm-zeta"), not_available("tm-alpha", Engine::Clippy)],
            "install clippy, then run tm baseline test again; each not-applicable rule above says why it was not run",
        ),
        (
            "unknown check only: the why itself, no install",
            3,
            vec![unknown_check("tm-alpha")],
            "ruff 0.4.1 does not know the check `XYZ123`: error: invalid value 'XYZ123' for '--select'",
        ),
        (
            "absent, unknown check and not applicable: install, then the why, then the pointer",
            3,
            vec![not_applicable("tm-gamma"), unknown_check("tm-alpha"), not_available("tm-zeta", Engine::SwiftLint)],
            "install swiftlint, then run tm baseline test again; \
             ruff 0.4.1 does not know the check `XYZ123`: error: invalid value 'XYZ123' for '--select'; \
             each not-applicable rule above says why it was not run",
        ),
        (
            "clippy's helper cargo missing: the why itself, no install",
            3,
            vec![not_available_why("tm-alpha", Engine::Clippy, "cargo not found on PATH")],
            "cargo not found on PATH",
        ),
        (
            "an engine with no entry missing git: the why itself, no install",
            3,
            vec![not_available_why("tm-alpha", Engine::ShellCheck, "git not found on PATH")],
            "git not found on PATH",
        ),
        (
            "clippy's own clippy-driver missing: install clippy",
            3,
            vec![not_available_why("tm-alpha", Engine::Clippy, "clippy-driver not found on PATH")],
            "install clippy, then run tm baseline test again",
        ),
    ];
    let wrong: Vec<String> = rows
        .into_iter()
        .filter_map(|(name, exit, results, want)| {
            let got = baseline_next(exit, &results);
            (got != want).then(|| format!("{name}:\n  got:  {got}\n  want: {want}"))
        })
        .collect();
    assert!(wrong.is_empty(), "rows wrong:\n{}", wrong.join("\n"));
}
