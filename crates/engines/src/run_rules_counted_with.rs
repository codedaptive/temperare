//! `run_rules_counted_with`: every rule of a rule file run by its engine through a host, rules that can share one
//! engine run sharing it, each with its outcome and its count, a too-old check and a check not in temperare's list of
//! its engine's codes naming the engine's version, a rule tm refuses `Failed` and a rule whose paths match no file of
//! its folder flagged, both without running.

use crate::engine_hits::engine_hits;
use crate::entry::too_old_why;
use crate::files_in_folder::files_in_folder;
use crate::host::Host;
use crate::not_on_path_why::not_on_path_why;
use crate::rule_folder::rule_folder;
use crate::run_rules_listed::run_rules_listed;
use crate::types::{EngineVersion, RuleRun, Stop};
use rules::{Engine, RelPath, Rule, RuleFile, RuleOutcome};
use std::path::Path;

/// Each rule's run, as `run_rules_versioned_at` gives its runs, in rule-file order, with the programs run and looked up
/// through `host`. The folder is resolved once and, when it resolves, its tracked and untracked-not-ignored files are
/// listed once (`files_in_folder`, None outside a git repository); `run_rules_listed` then fails each rule tm refuses
/// and flags each rule whose paths match none of them, with nothing run for either, and runs the rest, sharing a run
/// where rules can (`shared_groups`, `run_group`). A check the engine said it does not know, or one not in temperare's list of its engine's codes,
/// names the engine's version from `versions` (`stopped`).
/// When the folder cannot be resolved, nothing is listed, a rule tm refuses is still `Failed` with its refusal, a rule
/// with `only_with_base` and no base is still `NotAvailable` with `ONLY_WITH_BASE_WHY`, a rule whose engine is not
/// available is still `NotAvailable`, and every other rule is `Failed` with no exit code and nothing runs.
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
    let files = folder.as_deref().ok().and_then(files_in_folder);
    run_rules_listed(file, folder.as_deref(), only, host, versions, files.as_deref(), &engine_hits)
}

/// The outcome of `rule` when it stopped without findings. `NotAvailable` is `NotAvailable` with its why as it is;
/// `NotOnPath` is `NotAvailable` with `not_on_path_why` naming the program (`rule_run_of` sets the cause from the
/// stop, `not_available_cause_of`). A check the engine said it does not know is
/// `NotAvailable`, with `too_old_why` naming the engine by its rule-file name (`Engine::name`) and its version, the
/// first of `versions` for the rule's engine (`(version unknown)` when there is none or it is `None`). A check not in
/// temperare's list of its engine's codes (`NotListed`) is `NotAvailable` with `<engine> <version>: ` before what it
/// said, the engine and version found the same way. `NoFiles` is
/// `Findings` with none: `rule_run_of` settles a `NoFiles` itself, setting the rule's `no_files` flag, before it
/// would reach here, so this arm only keeps the match whole.
pub(crate) fn stopped(rule: &Rule, stop: Stop, versions: &[EngineVersion]) -> RuleOutcome {
    let (id, engine) = (rule.id.clone(), rule.engine);
    match stop {
        Stop::NotAvailable { why } => RuleOutcome::NotAvailable { rule: id, engine, why },
        Stop::NotOnPath { program } => RuleOutcome::NotAvailable { rule: id, engine, why: not_on_path_why(&program) },
        Stop::Failed { code, stderr } => RuleOutcome::Failed { rule: id, engine, code, stderr },
        Stop::NoFiles { .. } => RuleOutcome::Findings(Vec::new()),
        Stop::UnknownCheck { said } => {
            let why = too_old_why(engine.name(), version_of(engine, versions), &rule.check, &said);
            RuleOutcome::NotAvailable { rule: id, engine, why }
        }
        Stop::NotListed { said } => {
            let version = version_of(engine, versions).unwrap_or("(version unknown)");
            RuleOutcome::NotAvailable { rule: id, engine, why: format!("{} {version}: {said}", engine.name()) }
        }
    }
}

/// The version of `engine`: the first of `versions` for it, None when there is none or it is `None`.
fn version_of(engine: Engine, versions: &[EngineVersion]) -> Option<&str> {
    versions.iter().find(|v| v.engine == engine).and_then(|v| v.version.as_deref())
}
