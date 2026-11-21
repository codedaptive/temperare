//! ZUT for baseline_text.rs: the whole text, every line a literal.

use super::baseline_rule_result::BaselineRuleResult;
use super::baseline_text;
use super::case_verdict::CaseVerdict;
use rules::Engine;

fn result(set: &str, rule: &str, engine: Engine, failing: CaseVerdict, passing: CaseVerdict) -> BaselineRuleResult {
    BaselineRuleResult { set: set.to_string(), rule: rule.to_string(), engine, failing, passing }
}

fn broken(why: &str) -> CaseVerdict {
    CaseVerdict::Broken { why: why.to_string() }
}

fn not_available(why: &str) -> CaseVerdict {
    CaseVerdict::NotAvailable { why: why.to_string() }
}

fn not_applicable(why: &str) -> CaseVerdict {
    CaseVerdict::NotApplicable { why: why.to_string() }
}

#[test]
fn baseline_text_lines() {
    let mixed = vec![
        result("standard@1", "tm-zeta", Engine::Ruff, CaseVerdict::Holds, CaseVerdict::Holds),
        result("standard@1", "tm-alpha", Engine::Clippy, broken("gave no finding"), broken("gave x; expected none")),
        result("house@1", "tm-mid", Engine::SwiftLint, CaseVerdict::Holds, not_available("swiftlint is missing")),
        result("house@1", "tm-beta", Engine::AstGrep, not_available("ast-grep not found on PATH"), broken("boom")),
        result("house@1", "tm-gamma", Engine::Ruff, not_applicable("no params"), not_applicable("no params")),
    ];
    let rows = [
        ("no rules", vec![], "0 rules: 0 hold, 0 broken, 0 not available, 0 not applicable\n"),
        (
            "mixed, in the order given",
            mixed,
            "standard@1 tm-zeta (ruff): holds\n\
             standard@1 tm-alpha (clippy): broken\n\
             \x20 failing: broken: gave no finding\n\
             \x20 passing: broken: gave x; expected none\n\
             house@1 tm-mid (swiftlint): not available\n\
             \x20 passing: not available: swiftlint is missing\n\
             house@1 tm-beta (ast-grep): broken\n\
             \x20 failing: not available: ast-grep not found on PATH\n\
             \x20 passing: broken: boom\n\
             house@1 tm-gamma (ruff): not applicable\n\
             \x20 failing: not applicable: no params\n\
             \x20 passing: not applicable: no params\n\
             5 rules: 1 hold, 2 broken, 1 not available, 1 not applicable\n",
        ),
    ];
    for (name, results, expected) in rows {
        assert_eq!(baseline_text(&results), expected, "{name}");
    }
}
