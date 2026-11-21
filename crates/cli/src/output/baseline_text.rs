//! `baseline_text`: the results of `tm baseline test` as text.

use super::baseline_rule_result::BaselineRuleResult;
use super::case_verdict::CaseVerdict;
use std::fmt::Write;

/// Each rule in the order given, `<set> <rule> (<engine>): <verdict>`, then a line for each of its cases that does
/// not hold (failing first), then the counts line,
/// `<n> rules: <h> hold, <b> broken, <a> not available, <p> not applicable`; every line ends in a newline. The caller
/// adds the `next:` line.
pub(crate) fn baseline_text(results: &[BaselineRuleResult]) -> String {
    let mut text = String::new();
    let (mut holds, mut broken, mut not_available, mut not_applicable) = (0, 0, 0, 0);
    for r in results {
        let verdict = r.verdict();
        match verdict {
            "holds" => holds += 1,
            "broken" => broken += 1,
            "not available" => not_available += 1,
            "not applicable" => not_applicable += 1,
            // `verdict` gives only the four above; a verdict not named here is counted as none of them.
            _ => {}
        }
        // Writing to a String cannot fail.
        let _ = writeln!(text, "{} {} ({}): {verdict}", r.set, r.rule, r.engine.name());
        for (name, case) in [("failing", &r.failing), ("passing", &r.passing)] {
            if let Some((word, why)) = not_holding(case) {
                let _ = writeln!(text, "  {name}: {word}: {why}");
            }
        }
    }
    let _ = writeln!(
        text,
        "{} rules: {holds} hold, {broken} broken, {not_available} not available, {not_applicable} not applicable",
        results.len()
    );
    text
}

/// A case that does not hold as its verdict's words and its why; None when it holds.
fn not_holding(case: &CaseVerdict) -> Option<(&'static str, &str)> {
    match case {
        CaseVerdict::Holds => None,
        CaseVerdict::Broken { why } => Some(("broken", why)),
        CaseVerdict::NotAvailable { why } => Some(("not available", why)),
        CaseVerdict::NotApplicable { why } => Some(("not applicable", why)),
    }
}
