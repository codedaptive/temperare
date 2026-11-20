//! `assemble_report`: the report from the rules' outcomes, given the new-only filter.

use crate::{
    silenced_by_rule, split_by_exceptions, stderr_tail, Counts, Failed, NotApplicable, NotAvailable, Report, Silenced,
};
use procio::Oid;
use rules::{Date, Finding, Rule, RuleFile, RuleOutcome};

/// The why of a rule that reports only what a change adds, on a check with no base commit.
const NEEDS_BASE: &str = "reports only what a change adds; run with --base";

/// The why of a not-applicable rule whose reason is None; the rule is still listed, since it does not run.
const NO_REASON: &str = "no reason given";

/// The report of `outcomes`. Their findings go through `keep_new` first, so a finding it drops is neither kept nor
/// silenced nor counted; the rest are split by the rule file's exceptions (`RuleFile::exception_on`, given `day`).
/// `date` is carried as the report's date: the caller passes the "YYYY-MM-DD" text of `day`. The silenced findings are
/// counted by rule into `silenced_by_rule` (see `silenced_by_rule`). The `engines` list is left empty for the caller.
/// Findings, and silenced ones by their finding, are sorted by path, line, col (none first) and rule; not-available and
/// failed rules by rule. The sorts are stable, so the same input gives the same report. Each outcome carries its own
/// rule, so the tuple's name is not read. Pure: it runs nothing and reads no file.
///
/// `not_applicable` lists, unsorted, first each rule of `rules.not_applicable_rules` in that list's order, with its
/// `not_applicable` reason as why ("no reason given" when None); then, only when `base` is None, each rule of
/// `rules.rules` with `only_with_base`, in that list's order, with the why "reports only what a change adds; run with
/// --base". With no base, such a rule's outcomes give nothing: a finding whose own `rule` is its id, and a
/// not-available or failed outcome whose own `rule` is its id, are dropped before `keep_new` and the exceptions, so
/// none reaches findings, silenced, `silenced_by_rule`, not_available, failed or the counts. `counts.not_applicable` is
/// the list's length.
pub(crate) fn assemble_report(
    rules: &RuleFile,
    outcomes: Vec<(String, RuleOutcome)>,
    base: Option<Oid>,
    day: Date,
    date: String,
    keep_new: impl FnOnce(Vec<Finding>) -> Vec<Finding>,
) -> Report {
    let needs_base: Vec<&Rule> = match base {
        Some(_) => Vec::new(),
        None => rules.rules.iter().filter(|rule| rule.only_with_base).collect(),
    };
    let dropped = |rule: &str| needs_base.iter().any(|needed| needed.id == rule);
    let mut findings = Vec::new();
    let mut not_available = Vec::new();
    let mut failed = Vec::new();
    for (_, outcome) in outcomes {
        match outcome {
            RuleOutcome::Findings(found) => findings.extend(found.into_iter().filter(|f| !dropped(&f.rule))),
            RuleOutcome::NotAvailable { rule, .. } | RuleOutcome::Failed { rule, .. } if dropped(&rule) => {}
            RuleOutcome::NotAvailable { rule, engine, why } => not_available.push(NotAvailable { rule, engine, why }),
            RuleOutcome::Failed { rule, engine, code, stderr } => {
                failed.push(Failed { rule, engine, code, stderr_tail: stderr_tail(&stderr) })
            }
        }
    }
    let not_applicable = not_applicable(rules, &needs_base);
    let (mut kept, silenced) = split_by_exceptions(rules, keep_new(findings), day);
    let mut silenced: Vec<Silenced> =
        silenced.into_iter().map(|(finding, exception)| Silenced { finding, exception }).collect();
    kept.sort_by(|a, b| place(a).cmp(&place(b)));
    silenced.sort_by(|a, b| place(&a.finding).cmp(&place(&b.finding)));
    not_available.sort_by(|a, b| a.rule.cmp(&b.rule));
    failed.sort_by(|a, b| a.rule.cmp(&b.rule));
    Report {
        counts: Counts::of(&kept, silenced.len(), not_available.len(), failed.len(), not_applicable.len()),
        silenced_by_rule: silenced_by_rule(rules, &silenced),
        date,
        engines: Vec::new(),
        findings: kept,
        silenced,
        exceptions_in_force: rules.exceptions_in_force().len(),
        not_available,
        failed,
        base,
        not_applicable,
    }
}

/// The rule file's not-applicable rules with their reasons, then `needs_base` with `NEEDS_BASE`.
fn not_applicable(rules: &RuleFile, needs_base: &[&Rule]) -> Vec<NotApplicable> {
    rules
        .not_applicable_rules
        .iter()
        .map(|rule| (rule, rule.not_applicable.as_deref().unwrap_or(NO_REASON)))
        .chain(needs_base.iter().map(|rule| (*rule, NEEDS_BASE)))
        .map(|(rule, why)| NotApplicable { rule: rule.id.clone(), engine: rule.engine, why: why.to_string() })
        .collect()
}

/// The report's order key for a finding. `Option`'s order puts a finding with no line or col before any with one.
fn place(finding: &Finding) -> (&str, Option<u32>, Option<u32>, &str) {
    (finding.path.as_str(), finding.line(), finding.col, &finding.rule)
}
