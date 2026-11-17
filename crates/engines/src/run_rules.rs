//! `run_rules`: every rule of a rule file run by its engine, each with its outcome.

use crate::engine_hits::engine_hits;
use crate::findings_from::findings_from;
use crate::host::Host;
use crate::rule_folder::rule_folder;
use crate::types::Stop;
use rules::{RelPath, Rule, RuleFile, RuleOutcome};
use std::path::Path;

/// Each rule's id and outcome, in rule-file order. The rule file's folder is `file.folder`, relative to `repo` unless
/// absolute; every finding's path is relative to it. `only` empty checks the whole folder; otherwise findings are cut
/// to the paths it lists (engines still run over the folder). An absent engine is `NotAvailable` with
/// `availability`'s why; an engine that ran but gave no answer is `Failed`; otherwise `Findings`, empty when clean.
pub fn run_rules(file: &RuleFile, repo: &Path, only: &[RelPath]) -> Vec<(String, RuleOutcome)> {
    let host = Host { run: &procio::run, on_path: &|program: &str| procio::which(program).is_some() };
    run_rules_with(file, repo, only, &host)
}

/// `run_rules` with the programs run and looked up through `host`. The folder is resolved once. When it cannot be,
/// a rule whose engine is not available is still `NotAvailable`, and every other rule is `Failed` with no exit code
/// and nothing runs.
pub(crate) fn run_rules_with(file: &RuleFile, repo: &Path, only: &[RelPath], host: &Host) -> Vec<(String, RuleOutcome)> {
    let folder = rule_folder(file, repo).map_err(|err| Stop::Failed {
        code: None,
        stderr: format!("cannot resolve the rule file's folder {}: {err}", repo.join(&file.folder).display()),
    });
    file.rules
        .iter()
        .map(|rule| {
            let outcome = match engine_hits(rule, folder.as_deref(), host) {
                Ok(hits) => RuleOutcome::Findings(findings_from(file, rule, hits, only)),
                Err(stop) => stopped(rule, stop),
            };
            (rule.id.clone(), outcome)
        })
        .collect()
}

/// The outcome of `rule` when it stopped without findings.
fn stopped(rule: &Rule, stop: Stop) -> RuleOutcome {
    let (id, engine) = (rule.id.clone(), rule.engine);
    match stop {
        Stop::NotAvailable { why } => RuleOutcome::NotAvailable { rule: id, engine, why },
        Stop::Failed { code, stderr } => RuleOutcome::Failed { rule: id, engine, code, stderr },
    }
}
