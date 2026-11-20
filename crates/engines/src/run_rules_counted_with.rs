//! `run_rules_counted_with`: every rule of a rule file run by its engine through a host, each with its outcome and
//! the characters its check run printed, a too-old check naming its engine's version.

use crate::engine_hits::engine_hits;
use crate::entry::too_old_why;
use crate::entry_for::entry_for;
use crate::findings_from::findings_from;
use crate::host::Host;
use crate::rule_folder::rule_folder;
use crate::types::{EngineVersion, RuleRun, Stop};
use rules::{RelPath, Rule, RuleFile, RuleOutcome};
use std::path::Path;

/// Each rule's run, as `run_rules_counted` gives it, with the programs run and looked up through `host`, and each
/// rule's count read from `host.raw_chars`, cleared before the rule runs. A check the engine said it does not know
/// names the engine's version from `versions` (`stopped`). The folder is resolved once. When it cannot be, a rule
/// whose engine is not available is still `NotAvailable`, and every other rule is `Failed` with no exit code and
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
    file.rules
        .iter()
        .map(|rule| {
            host.raw_chars.set(None);
            let outcome = match engine_hits(rule, folder.as_deref(), host) {
                Ok(hits) => RuleOutcome::Findings(findings_from(file, rule, hits, only)),
                Err(stop) => stopped(rule, stop, versions),
            };
            RuleRun { rule: rule.id.clone(), engine: rule.engine, outcome, raw_chars: host.raw_chars.get() }
        })
        .collect()
}

/// The outcome of `rule` when it stopped without findings. A check the engine said it does not know is
/// `NotAvailable`, with `too_old_why` naming the engine by its entry's name (`entry_for`) and its version, the first
/// of `versions` for the rule's engine (`(version unknown)` when there is none or it is `None`).
pub(crate) fn stopped(rule: &Rule, stop: Stop, versions: &[EngineVersion]) -> RuleOutcome {
    let (id, engine) = (rule.id.clone(), rule.engine);
    match stop {
        Stop::NotAvailable { why } => RuleOutcome::NotAvailable { rule: id, engine, why },
        Stop::Failed { code, stderr } => RuleOutcome::Failed { rule: id, engine, code, stderr },
        Stop::UnknownCheck { said } => {
            let version = versions.iter().find(|v| v.engine == engine).and_then(|v| v.version.as_deref());
            let why = too_old_why(entry_for(engine).name, version, &rule.check, &said);
            RuleOutcome::NotAvailable { rule: id, engine, why }
        }
    }
}
