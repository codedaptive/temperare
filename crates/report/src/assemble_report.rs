//! `assemble_report`: the report from the rules' outcomes, given the new-only filter.

use crate::{
    out_of_play, silenced_by_rule, split_by_exceptions, stderr_tail, Counts, Expired, Failed, NotAvailable, Report,
    Silenced,
};
use procio::Oid;
use rules::{Date, Exception, Finding, RuleFile, RuleOutcome};

/// The report of `outcomes`. Their findings go through `keep_new` first, so a finding it drops is neither kept nor
/// silenced nor counted; the rest are split by the rule file's exceptions (`RuleFile::exception_on`, given `day`).
/// `date` is carried as the report's date: the caller passes the "YYYY-MM-DD" text of `day`. The silenced findings are
/// counted by rule into `silenced_by_rule` (see `silenced_by_rule`). The `engines` list is left empty for the caller.
/// `expired` lists `rules.exceptions_expired(day)` in rule-file order, `exceptions_in_force` counts the rest, both
/// judged on the same `day` the exceptions silence by; `counts.expired` is the list's length.
/// Findings, and silenced ones by their finding, are sorted by path, line, col (none first) and rule; not-available and
/// failed rules by rule. The sorts are stable, so the same input gives the same report. Each outcome carries its own
/// rule, so the tuple's name is not read. Pure: it runs nothing and reads no file.
///
/// `covers_nothing` and `not_applicable` are `out_of_play`'s lists, with `no_base` when `base` is None (see
/// `out_of_play` for which rules they hold and in what order). The outcomes of the rules in its third list are dropped,
/// a finding by its own `rule` and a not-available or failed outcome by its own `rule`, before `keep_new` and the
/// exceptions, so none reaches findings, silenced, `silenced_by_rule`, not_available, failed or the counts.
/// `counts.covers_nothing` and `counts.not_applicable` are the lists' lengths.
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
    let expired: Vec<Expired> = rules.exceptions_expired(day).into_iter().filter_map(expired_entry).collect();
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
            expired.len(),
        ),
        silenced_by_rule: silenced_by_rule(rules, &silenced),
        date,
        engines: Vec::new(),
        findings: kept,
        silenced,
        exceptions_in_force: rules.exceptions.len() - expired.len(),
        not_available,
        failed,
        base,
        covers_nothing,
        not_applicable,
        expired,
    }
}

/// The report's entry for an expired exception; None only without `expires`, which `exceptions_expired` never lists,
/// so nothing is dropped.
fn expired_entry(exception: &Exception) -> Option<Expired> {
    let date = exception.expires?;
    Some(Expired {
        rule: exception.rule.clone(),
        path: exception.path.clone(),
        what: exception.what.clone(),
        expires: date.to_string(),
        reason: exception.reason.clone(),
        approved: exception.approved.clone(),
    })
}

/// The report's order key for a finding. `Option`'s order puts a finding with no line or col before any with one.
fn place(finding: &Finding) -> (&str, Option<u32>, Option<u32>, &str) {
    (finding.path.as_str(), finding.line(), finding.col, &finding.rule)
}
