//! `run_rules_counted_with`: every rule of a rule file run by its engine through a host, rules that can share one
//! engine run sharing it, each with its outcome and its count, a too-old check and a check not in temperare's list of
//! its engine's codes naming the engine's version, a rule tm refuses `Failed` and a rule whose paths match no file of
//! its folder flagged, both without running.

use crate::engine_hits::engine_hits;
use crate::files_in_folder::files_in_folder;
use crate::host::Host;
use crate::rule_folder::rule_folder;
use crate::run_rules_listed::run_rules_listed;
use crate::types::{EngineVersion, RuleRun, Stop};
use rules::{RelPath, RuleFile};
use std::path::Path;

/// Each rule's run, as `run_rules_versioned_at` gives its runs, in rule-file order, with the programs run and looked up
/// through `host`. The folder is resolved once and, when it resolves, its tracked and untracked-not-ignored files are
/// listed once (`files_in_folder`, None outside a git repository); `run_rules_listed` then fails each rule tm refuses
/// and flags each rule whose paths match none of them, with nothing run for either, and runs the rest, sharing a run
/// where rules can (`shared_groups`, `run_group`). A check the engine said it does not know, or one not in temperare's list of its engine's codes,
/// names the engine's version from `versions` (`stopped`).
/// When the folder cannot be resolved, nothing is listed, a rule tm refuses is still `Failed` with its refusal, a rule
/// with `only_with_base` and no base is still `NotAvailable` with `rules::ONLY_WITH_BASE_WHY`, a rule whose engine
/// is not available is still `NotAvailable`, and every other rule is `Failed` with no exit code and nothing runs.
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
