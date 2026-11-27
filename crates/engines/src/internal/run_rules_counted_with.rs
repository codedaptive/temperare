//! `run_rules_counted_with`: every rule of a rule file run by its engine through a host, rules that can share one
//! engine run sharing it, each with its outcome and its count, a too-old check and a check not in cctao's list of
//! its engine's codes naming the engine's version, a rule cctao refuses `Failed` and a rule whose paths match no file of
//! its folder flagged, both without running.

use crate::engine_hits::engine_hits;
use crate::files_in_folder::files_in_folder;
use crate::host::Host;
use crate::rule_folder::rule_folder;
use crate::run_rules_listed::run_rules_listed;
use crate::types::{EngineVersion, RuleRun, Stop};
use rules::{RelPath, Rule, RuleFile};
use std::path::Path;

/// Each rule's run, as `run_rules_versioned_at` gives its runs, in rule-file order, with the programs run and looked up
/// through `host`. The folder is resolved once and, when it resolves, its tracked and untracked-not-ignored regular
/// files are listed once (`files_in_folder`, None outside a git repository); `run_rules_listed` then fails each rule
/// cctao refuses and flags each rule whose paths match none of them, with nothing run for either, and runs the rest,
/// sharing a run where rules can (`shared_groups`, `run_group`). A check the engine said it does not know, or one not
/// in cctao's list of its engine's codes, names the engine's version from `versions` (`stopped`). A rule whose
/// engine's version in `versions` is older than the release its list of codes was taken from is not available
/// (`list_floor`, inside `engine_hits`, after the no-file and base steps).
/// When the folder cannot be resolved, nothing is listed, a rule cctao refuses is still `Failed` with its refusal, a rule
/// with `only_with_base` and no base is still `NotAvailable` with `rules::ONLY_WITH_BASE_WHY`, a rule whose engine
/// is not available, or older than its list, is still `NotAvailable`, and every other rule is `Failed` with no exit
/// code and nothing runs. `run_rules_versioned_with` gives no version for an unresolved folder, so in production the
/// floor never fires there.
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
    // `HitsFn` is a `'static` trait object, so the flow owns its copy of the versions rather than borrowing them.
    let floors = versions.to_vec();
    let hits = move |rule: &Rule, folder: Result<&Path, &Stop>, host: &Host| engine_hits(rule, folder, host, &floors);
    run_rules_listed(file, folder.as_deref(), only, host, versions, files.as_deref(), &hits)
}
