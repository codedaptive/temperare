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
/// `not_applicable` is in rule-file order: it walks `RuleFile::rules_in_order` (by `position`; rules sharing one keep
/// `rules` first, then `not_applicable_rules`). A rule that sits in `rules.not_applicable_rules`, told by its address
/// in that list and not by its reason, is listed with its `not_applicable` reason as why ("no reason given" when None),
/// even if it has `only_with_base`. Otherwise, only when `base` is None, a rule with `only_with_base` is listed with
/// the why "reports only what a change adds; run with --base". With no base, such a rule of `rules.rules` gives
/// nothing: a finding whose own `rule` is its id, and a not-available or failed outcome whose own `rule` is its id,
/// are dropped before `keep_new` and the exceptions, so none reaches findings, silenced, `silenced_by_rule`,
/// not_available, failed or the counts. `counts.not_applicable` is the list's length.
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
    let not_applicable = not_applicable(rules, base.is_none());
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

/// The rules out of play in rule-file order: each of `rules.not_applicable_rules` with its reason, and when `no_base`,
/// each other rule with `only_with_base` with `NEEDS_BASE`. Membership is by address, so a None reason still counts.
fn not_applicable(rules: &RuleFile, no_base: bool) -> Vec<NotApplicable> {
    let set_aside = rules.not_applicable_rules.as_ptr_range();
    rules
        .rules_in_order()
        .into_iter()
        .filter_map(|rule| {
            let why = if set_aside.contains(&std::ptr::from_ref(rule)) {
                rule.not_applicable.as_deref().unwrap_or(NO_REASON)
            } else if no_base && rule.only_with_base {
                NEEDS_BASE
            } else {
                return None;
            };
            Some(NotApplicable { rule: rule.id.clone(), engine: rule.engine, why: why.to_string() })
        })
        .collect()
}

/// The report's order key for a finding. `Option`'s order puts a finding with no line or col before any with one.
fn place(finding: &Finding) -> (&str, Option<u32>, Option<u32>, &str) {
    (finding.path.as_str(), finding.line(), finding.col, &finding.rule)
}
