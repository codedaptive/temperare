//! `assemble_report`: the report from the rules' outcomes, given the new-only filter.

use crate::{split_by_exceptions, stderr_tail, Counts, Failed, NotAvailable, Report, Silenced};
use procio::Oid;
use rules::{Finding, RuleFile, RuleOutcome};

/// The report of `outcomes`. Their findings go through `keep_new` first, so a finding it drops is neither kept nor
/// silenced nor counted; the rest are split by the rule file's exceptions. Findings, and silenced ones by their
/// finding, are sorted by path, line, col (none first) and rule; not-available and failed rules by rule. The sorts
/// are stable, so the same input gives the same report. Each outcome carries its own rule, so the tuple's name is
/// not read. Pure: it runs nothing and reads no file.
pub(crate) fn assemble_report(
    rules: &RuleFile,
    outcomes: Vec<(String, RuleOutcome)>,
    base: Option<Oid>,
    keep_new: impl FnOnce(Vec<Finding>) -> Vec<Finding>,
) -> Report {
    let mut findings = Vec::new();
    let mut not_available = Vec::new();
    let mut failed = Vec::new();
    for (_, outcome) in outcomes {
        match outcome {
            RuleOutcome::Findings(found) => findings.extend(found),
            RuleOutcome::NotAvailable { rule, engine, why } => not_available.push(NotAvailable { rule, engine, why }),
            RuleOutcome::Failed { rule, engine, code, stderr } => {
                failed.push(Failed { rule, engine, code, stderr_tail: stderr_tail(&stderr) })
            }
        }
    }
    let (mut kept, silenced) = split_by_exceptions(rules, keep_new(findings));
    let mut silenced: Vec<Silenced> =
        silenced.into_iter().map(|(finding, exception)| Silenced { finding, exception }).collect();
    kept.sort_by(|a, b| place(a).cmp(&place(b)));
    silenced.sort_by(|a, b| place(&a.finding).cmp(&place(&b.finding)));
    not_available.sort_by(|a, b| a.rule.cmp(&b.rule));
    failed.sort_by(|a, b| a.rule.cmp(&b.rule));
    Report {
        counts: Counts::of(&kept, silenced.len(), not_available.len(), failed.len()),
        findings: kept,
        silenced,
        exceptions_in_force: rules.exceptions_in_force().len(),
        not_available,
        failed,
        base,
    }
}

/// The report's order key for a finding.
fn place(finding: &Finding) -> (&str, u32, Option<u32>, &str) {
    (finding.path.as_str(), finding.line, finding.col, &finding.rule)
}
