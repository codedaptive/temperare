//! `run_rules_listed`: every rule of a rule file run through a host, a rule tm refuses or whose paths match no file
//! of its folder settled before anything runs.

use crate::host::Host;
use crate::no_file_match::{no_file_match, NO_FILE_MATCH_WHY};
use crate::refused::refused;
use crate::rule_run_of::rule_run_of;
use crate::run_group::run_group;
use crate::shared_groups::shared_groups;
use crate::types::{EngineVersion, HitsFn, RuleRun, Stop};
use rules::{RelPath, Rule, RuleFile, RuleOutcome};
use std::path::Path;

/// Each rule's run, in rule-file order, `files` being the tracked and untracked-not-ignored files of the rule file's
/// folder relative to it (None when git could not list them), `hits` being one rule's flow (`engine_hits` in
/// production). Each rule is settled in one order, the first that applies deciding it:
/// 1. a rule `refused` refuses (its check, or a param its engine does not take) is its `Failed` through
///    `rule_run_of`, with no count and no `no_files` flag, whatever its files: no run could make it clean;
/// 2. with a listing, a rule that `no_file_match` says matches none of the files: no findings, no count, `no_files`
///    set to `NO_FILE_MATCH_WHY`; with no listing no rule is settled so. Inside `hits`, `engine_hits` then settles
///    a ShellCheck rule with no script in its paths, or a cargo-audit rule with no Cargo.lock in them, as
///    `Stop::NoFiles`, before availability;
/// 2b. inside `hits`, a rule with `only_with_base` and no `host.base`: `engine_hits` gives it `Stop::NotAvailable`
///    with `rules::ONLY_WITH_BASE_WHY`, so its run is `NotAvailable` with cause `Other` and no count;
/// 3. an engine that is not available, inside `hits`;
/// 4. the run, inside `hits`.
///
/// Nothing runs or is looked up for a rule settled by 1 or by `no_file_match`, not even its engine's availability,
/// and it joins no shared group, so no shared run's union holds its check. A rule settled inside `hits` before
/// its check runs (ShellCheck's no script, cargo-audit's no Cargo.lock, 2b, 3) never runs its check, and its check is never put in a shared run:
/// `run_group`'s probe pass stops it before its check run. The other rules are split into groups that
/// can share one engine run (`shared_groups`): a group of one runs alone through `hits` with `host`, its count read
/// from `host.raw_chars`, cleared before it runs; a larger group runs through `run_group`, which gives `hits` each
/// rule's own host. Each rule's flow result becomes its run through `rule_run_of`.
pub(crate) fn run_rules_listed(
    file: &RuleFile,
    folder: Result<&Path, &Stop>,
    only: &[RelPath],
    host: &Host,
    versions: &[EngineVersion],
    files: Option<&[RelPath]>,
    hits: &HitsFn,
) -> Vec<RuleRun> {
    let mut runs: Vec<(usize, RuleRun)> = Vec::new();
    let mut running: Vec<usize> = Vec::new();
    for (at, rule) in file.rules.iter().enumerate() {
        if let Some(stop) = refused(rule) {
            runs.push((at, rule_run_of(file, rule, Err(stop), only, versions, None)));
        } else if files.is_some_and(|files| no_file_match(rule, files)) {
            let run = RuleRun::new(rule.id.clone(), rule.engine, RuleOutcome::Findings(Vec::new()), None);
            runs.push((at, run.with_no_files(NO_FILE_MATCH_WHY)));
        } else {
            running.push(at);
        }
    }
    let rules: Vec<Rule> = running.iter().map(|at| file.rules[*at].clone()).collect();
    for group in shared_groups(&rules) {
        if let [at] = group[..] {
            runs.push((running[at], run_alone(file, &rules[at], folder, only, host, versions, hits)));
            continue;
        }
        let members: Vec<&Rule> = group.iter().map(|at| &rules[*at]).collect();
        let ats = group.iter().map(|at| running[*at]);
        runs.extend(ats.zip(run_group(file, &members, folder, only, host, versions, hits)));
    }
    runs.sort_by_key(|(at, _)| *at);
    runs.into_iter().map(|(_, run)| run).collect()
}

/// `rule` run on its own by `hits` through `host`, its count read from `host.raw_chars`, cleared first.
fn run_alone(
    file: &RuleFile,
    rule: &Rule,
    folder: Result<&Path, &Stop>,
    only: &[RelPath],
    host: &Host,
    versions: &[EngineVersion],
    hits: &HitsFn,
) -> RuleRun {
    host.raw_chars.set(None);
    let got = hits(rule, folder, host);
    rule_run_of(file, rule, got, only, versions, host.raw_chars.get())
}
