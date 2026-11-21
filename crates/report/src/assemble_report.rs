//! `assemble_report`: the report from the rules' outcomes, given the new-only filter.

use crate::{
    silenced_by_rule, split_by_exceptions, stderr_tail, Counts, CoversNothing, Failed, NotApplicable, NotAvailable,
    Report, Silenced,
};
use procio::Oid;
use rules::{Date, Finding, RuleFile, RuleOutcome};

/// The why of a rule that reports only what a change adds, on a check with no base commit.
const NEEDS_BASE: &str = "reports only what a change adds; run with --base";

/// The why of a not-applicable rule whose reason is None; the rule is still listed, since it does not run.
const NO_REASON: &str = "no reason given";

/// The start of every baseline set's rule id; a repository's own ids never start with it (rules' `check_id`).
const BASELINE_PREFIX: &str = "tm-";

/// The report of `outcomes`. Their findings go through `keep_new` first, so a finding it drops is neither kept nor
/// silenced nor counted; the rest are split by the rule file's exceptions (`RuleFile::exception_on`, given `day`).
/// `date` is carried as the report's date: the caller passes the "YYYY-MM-DD" text of `day`. The silenced findings are
/// counted by rule into `silenced_by_rule` (see `silenced_by_rule`). The `engines` list is left empty for the caller.
/// Findings, and silenced ones by their finding, are sorted by path, line, col (none first) and rule; not-available and
/// failed rules by rule. The sorts are stable, so the same input gives the same report. Each outcome carries its own
/// rule, so the tuple's name is not read. Pure: it runs nothing and reads no file.
///
/// `covers_nothing` and `not_applicable` come from one walk of `RuleFile::rules_in_order` (by `position`; rules sharing
/// one keep `rules` first, then `not_applicable_rules`), so both are in rule-file order and a rule is listed once, by
/// the first of these that holds: a rule that sits in `rules.not_applicable_rules`, told by its address in that list
/// and not by its reason, is not applicable with its own reason ("no reason given" when None); else, when `base` is
/// None, a rule with `only_with_base` is not applicable with the why "reports only what a change adds; run with
/// --base"; else a rule flagged in `no_files` (rule id, why; the first pair for an id counts) is not applicable with
/// that why when its id starts "tm-" (a baseline set's), and otherwise (the repository's own) covers nothing. A flagged
/// id that names no rule of the file is ignored: there is no engine or rule-file place to list it with. A rule of
/// `rules.rules` listed by the second or third gives nothing: a finding whose own `rule` is its id, and a not-available
/// or failed outcome whose own `rule` is its id, are dropped before `keep_new` and the exceptions, so none reaches
/// findings, silenced, `silenced_by_rule`, not_available, failed or the counts. `counts.covers_nothing` and
/// `counts.not_applicable` are the lists' lengths.
pub(crate) fn assemble_report(
    rules: &RuleFile,
    outcomes: Vec<(String, RuleOutcome)>,
    base: Option<Oid>,
    day: Date,
    date: String,
    no_files: &[(String, String)],
    keep_new: impl FnOnce(Vec<Finding>) -> Vec<Finding>,
) -> Report {
    let (covers_nothing, not_applicable, dropped_ids) = out_of_play(rules, base.is_none(), no_files);
    let dropped = |rule: &str| dropped_ids.contains(&rule);
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
    let (mut kept, silenced) = split_by_exceptions(rules, keep_new(findings), day);
    let mut silenced: Vec<Silenced> =
        silenced.into_iter().map(|(finding, exception)| Silenced { finding, exception }).collect();
    kept.sort_by(|a, b| place(a).cmp(&place(b)));
    silenced.sort_by(|a, b| place(&a.finding).cmp(&place(&b.finding)));
    not_available.sort_by(|a, b| a.rule.cmp(&b.rule));
    failed.sort_by(|a, b| a.rule.cmp(&b.rule));
    Report {
        counts: Counts::of(
            &kept,
            silenced.len(),
            not_available.len(),
            failed.len(),
            not_applicable.len(),
            covers_nothing.len(),
        ),
        silenced_by_rule: silenced_by_rule(rules, &silenced),
        date,
        engines: Vec::new(),
        findings: kept,
        silenced,
        exceptions_in_force: rules.exceptions_in_force().len(),
        not_available,
        failed,
        base,
        covers_nothing,
        not_applicable,
    }
}

/// The rules that cover nothing, the rules not applied, and the ids of the rules of `rules.rules` among them, whose
/// outcomes the report drops; the walk and its order are `assemble_report`'s. Membership of `not_applicable_rules` is
/// by address, so a None reason still counts.
fn out_of_play<'a>(
    rules: &'a RuleFile,
    no_base: bool,
    no_files: &[(String, String)],
) -> (Vec<CoversNothing>, Vec<NotApplicable>, Vec<&'a str>) {
    let set_aside = rules.not_applicable_rules.as_ptr_range();
    let (mut covers_nothing, mut not_applicable, mut dropped) = (Vec::new(), Vec::new(), Vec::new());
    for rule in rules.rules_in_order() {
        let flag = no_files.iter().find(|(id, _)| *id == rule.id).map(|(_, why)| why.as_str());
        let listed = |why: &str| NotApplicable { rule: rule.id.clone(), engine: rule.engine, why: why.to_string() };
        match (set_aside.contains(&std::ptr::from_ref(rule)), no_base && rule.only_with_base, flag) {
            (true, _, _) => {
                not_applicable.push(listed(rule.not_applicable.as_deref().unwrap_or(NO_REASON)));
                continue;
            }
            (_, true, _) => not_applicable.push(listed(NEEDS_BASE)),
            (_, _, Some(why)) if rule.id.starts_with(BASELINE_PREFIX) => not_applicable.push(listed(why)),
            (_, _, Some(_)) => covers_nothing.push(CoversNothing { rule: rule.id.clone(), engine: rule.engine }),
            (_, _, None) => continue,
        }
        dropped.push(rule.id.as_str());
    }
    (covers_nothing, not_applicable, dropped)
}

/// The report's order key for a finding. `Option`'s order puts a finding with no line or col before any with one.
fn place(finding: &Finding) -> (&str, Option<u32>, Option<u32>, &str) {
    (finding.path.as_str(), finding.line(), finding.col, &finding.rule)
}
