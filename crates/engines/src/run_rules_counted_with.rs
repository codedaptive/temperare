//! `run_rules_counted_with`: every rule of a rule file run by its engine through a host, rules that can share one
//! engine run sharing it, each with its outcome and its count, a too-old check naming its engine's version.

use crate::engine_hits::engine_hits;
use crate::entry::too_old_why;
use crate::findings_from::findings_from;
use crate::host::Host;
use crate::rule_folder::rule_folder;
use crate::run_group::run_group;
use crate::shared_groups::shared_groups;
use crate::types::{EngineVersion, RuleRun, Stop};
use rules::{RelPath, Rule, RuleFile, RuleOutcome};
use std::path::Path;

/// Each rule's run, as `run_rules_counted` gives it, in rule-file order, with the programs run and looked up through
/// `host`. The rules are split into groups that can share one engine run (`shared_groups`). A group of one runs alone,
/// its count read from `host.raw_chars`, cleared before it runs; a larger group runs through `run_group`, one counted
/// run per shared run: `Some` on the first rule it settled, `None` on the rest. A check the engine said it does not
/// know names the engine's version from `versions` (`stopped`). The folder is resolved once. When it cannot be, a
/// rule whose engine is not available is still `NotAvailable`, and every other rule is `Failed` with no exit code and
/// nothing runs.
pub(crate) fn run_rules_counted_with(
    file: &RuleFile,
    repo: &Path,
    only: &[RelPath],
    host: &Host,
    versions: &[EngineVersion],
) -> Vec<RuleRun> {
    let folder = rule_folder(file, repo).map_err(|err| Stop::Failed {
        code: None,
        stderr: format!("cannot resolve the rule file's folder {}: {err}", repo.join(&file.folder).display()),
    });
    let mut runs: Vec<(usize, RuleRun)> = Vec::new();
    for group in shared_groups(&file.rules) {
        if let [at] = group[..] {
            runs.push((at, run_alone(file, &file.rules[at], folder.as_deref(), only, host, versions)));
            continue;
        }
        let rules: Vec<&Rule> = group.iter().map(|at| &file.rules[*at]).collect();
        runs.extend(group.into_iter().zip(run_group(file, &rules, folder.as_deref(), only, host, versions)));
    }
    runs.sort_by_key(|(at, _)| *at);
    runs.into_iter().map(|(_, run)| run).collect()
}

/// `rule` run on its own through `host`, its count read from `host.raw_chars`, cleared first.
fn run_alone(
    file: &RuleFile,
    rule: &Rule,
    folder: Result<&Path, &Stop>,
    only: &[RelPath],
    host: &Host,
    versions: &[EngineVersion],
) -> RuleRun {
    host.raw_chars.set(None);
    let outcome = match engine_hits(rule, folder, host) {
        Ok(hits) => RuleOutcome::Findings(findings_from(file, rule, hits, only)),
        Err(stop) => stopped(rule, stop, versions),
    };
    RuleRun { rule: rule.id.clone(), engine: rule.engine, outcome, raw_chars: host.raw_chars.get() }
}

/// The outcome of `rule` when it stopped without findings. A check the engine said it does not know is
/// `NotAvailable`, with `too_old_why` naming the engine by its rule-file name (`Engine::name`) and its version, the
/// first of `versions` for the rule's engine (`(version unknown)` when there is none or it is `None`).
pub(crate) fn stopped(rule: &Rule, stop: Stop, versions: &[EngineVersion]) -> RuleOutcome {
    let (id, engine) = (rule.id.clone(), rule.engine);
    match stop {
        Stop::NotAvailable { why } => RuleOutcome::NotAvailable { rule: id, engine, why },
        Stop::Failed { code, stderr } => RuleOutcome::Failed { rule: id, engine, code, stderr },
        Stop::UnknownCheck { said } => {
            let version = versions.iter().find(|v| v.engine == engine).and_then(|v| v.version.as_deref());
            let why = too_old_why(engine.name(), version, &rule.check, &said);
            RuleOutcome::NotAvailable { rule: id, engine, why }
        }
    }
}
