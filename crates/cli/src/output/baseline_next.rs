//! `baseline_next`: the text after `next: ` for `tm baseline test`.

use super::baseline_rule_result::BaselineRuleResult;
use super::case_verdict::CaseVerdict;
use super::engine_program_missing::engine_program_missing;

/// What the user does next for `exit`: 0 has nothing to do; 1 fixes the broken rules or their cases; 3 gives the
/// parts that apply, joined by `; `: install the engine of each not-available rule whose case says one of the engine's
/// own programs is missing (`engine_program_missing`), then each other not-available case's why once (a missing
/// helper such as cargo included), then read why each not-applicable rule was not run.
/// `baseline_exit` gives no other code; any other points at the output above.
pub(crate) fn baseline_next(exit: u8, results: &[BaselineRuleResult]) -> String {
    match exit {
        0 => "nothing to do".to_string(),
        1 => "fix each broken rule or its cases above".to_string(),
        3 => not_run_parts(results).join("; "),
        _ => "see the output above".to_string(),
    }
}

/// The parts of exit 3's next text: the engines to install, then the other not-available whys, then the pointer at
/// the not-applicable rules' whys when one is not applicable.
fn not_run_parts(results: &[BaselineRuleResult]) -> Vec<String> {
    let (engines, whys) = installs_and_whys(results);
    let mut parts = Vec::new();
    if !engines.is_empty() {
        parts.push(format!("install {}, then run tm baseline test again", engines.join(", ")));
    }
    parts.extend(whys.into_iter().map(str::to_string));
    if results.iter().any(|r| r.verdict() == "not applicable") {
        parts.push("each not-applicable rule above says why it was not run".to_string());
    }
    parts
}

/// From the not-available rules' not-available cases, failing case then passing case: the engine names whose why
/// says one of the engine's own programs is missing, and every other why; each list holds each entry once, in
/// first-seen order.
fn installs_and_whys(results: &[BaselineRuleResult]) -> (Vec<&'static str>, Vec<&str>) {
    let mut engines = Vec::new();
    let mut whys = Vec::new();
    for r in results.iter().filter(|r| r.verdict() == "not available") {
        for case in [&r.failing, &r.passing] {
            match case {
                CaseVerdict::NotAvailable { why } if engine_program_missing(r.engine, why) => {
                    push_once(&mut engines, r.engine.name())
                }
                CaseVerdict::NotAvailable { why } => push_once(&mut whys, why.as_str()),
                _ => {}
            }
        }
    }
    (engines, whys)
}

fn push_once<'a>(list: &mut Vec<&'a str>, item: &'a str) {
    if !list.contains(&item) {
        list.push(item);
    }
}
