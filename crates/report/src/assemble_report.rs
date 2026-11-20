//! `assemble_report`: the report from the rules' outcomes, given the new-only filter.

use crate::{silenced_by_rule, split_by_exceptions, stderr_tail, Counts, Failed, NotAvailable, Report, Silenced};
use procio::Oid;
use rules::{Date, Finding, RuleFile, RuleOutcome};

/// The report of `outcomes`. Their findings go through `keep_new` first, so a finding it drops is neither kept nor
/// silenced nor counted; the rest are split by the rule file's exceptions (`RuleFile::exception_on`, given `day`).
/// `date` is carried as the report's date: the caller passes the "YYYY-MM-DD" text of `day`. The silenced findings are
/// counted by rule into `silenced_by_rule` (see `silenced_by_rule`). The `engines` list is left empty for the caller.
/// Findings, and silenced ones by their finding, are sorted by path, line, col (none first) and rule; not-available and
/// failed rules by rule. The sorts are stable, so the same input gives the same report. Each outcome carries its own
/// rule, so the tuple's name is not read. Pure: it runs nothing and reads no file.
pub(crate) fn assemble_report(
    rules: &RuleFile,
    outcomes: Vec<(String, RuleOutcome)>,
    base: Option<Oid>,
    day: Date,
    date: String,
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
    let (mut kept, silenced) = split_by_exceptions(rules, keep_new(findings), day);
    let mut silenced: Vec<Silenced> =
        silenced.into_iter().map(|(finding, exception)| Silenced { finding, exception }).collect();
    kept.sort_by(|a, b| place(a).cmp(&place(b)));
    silenced.sort_by(|a, b| place(&a.finding).cmp(&place(&b.finding)));
    not_available.sort_by(|a, b| a.rule.cmp(&b.rule));
    failed.sort_by(|a, b| a.rule.cmp(&b.rule));
    Report {
        counts: Counts::of(&kept, silenced.len(), not_available.len(), failed.len()),
        silenced_by_rule: silenced_by_rule(rules, &silenced),
        date,
        engines: Vec::new(),
        findings: kept,
        silenced,
        exceptions_in_force: rules.exceptions_in_force().len(),
        not_available,
        failed,
        base,
    }
}

/// The report's order key for a finding. `Option`'s order puts a finding with no line or col before any with one.
fn place(finding: &Finding) -> (&str, Option<u32>, Option<u32>, &str) {
    (finding.path.as_str(), finding.line(), finding.col, &finding.rule)
}
