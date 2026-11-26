//! `engine_hits_with`: one rule's hits from its engine's entry, with its no-file outcome (`no_file_listed`, through a
//! given lister of scripts for ShellCheck) settled before availability, a rule that needs a base settled without one,
//! and an engine older than the release its list of codes was taken from settled before it is looked up.

use crate::entry::list_floor;
use crate::entry_of::entry_of;
use crate::host::Host;
use crate::no_file_listed::no_file_listed;
use crate::types::{EngineVersion, Hit, Stop};
use crate::unavailable::unavailable;
use rules::{Rule, ONLY_WITH_BASE_WHY};
use std::path::{Path, PathBuf};

/// The hits of `rule` under `folder`: the rule file's canonical folder, or the `Stop` its resolving gave. In the
/// order a rule is settled (refusals, which the caller settles first; no file; not available; the run). Departures:
/// ESLint and RuboCop settle no file in their flow, after availability, since their config decides the files; ast-grep
/// has no no-file step of its own; a listing error in step 1 falls through to the steps below, where the engine's flow
/// lists again and stops with it (dart's flow lists nothing: its own `repo_top` call stops when git cannot give the
/// top):
/// 1. a rule with a resolved folder is `NoFiles`, with its why, when `no_file_listed` (ShellCheck listing through
///    `scripts`; its doc lists each engine's lister) finds no file of its engine's that the rule applies to. Nothing
///    is looked up and the flow never runs;
/// 1b. a rule with `only_with_base` and no `host.base` is `NotAvailable` with `rules::ONLY_WITH_BASE_WHY`, whatever its
///    engine and whatever `folder` is: nothing is looked up on PATH and the flow never runs;
/// 1c. an engine whose version in `versions` (probed once, `engine_versions`) is older than the release temperare's
///    list of its codes was taken from (`list_floor`) is `NotAvailable` with the floor's why (cause `Other`): nothing
///    is looked up and the flow never runs. It comes after the no-file steps and the base step, so an old engine with
///    no file of its kind is still `NoFiles`;
/// 2. an engine that `unavailable` says cannot be run here gets that stop: `NotOnPath` for a program it needs not
///    found, `NotAvailable` for one never available. ESLint's program counts as found when the resolved folder holds
///    `node_modules/.bin/eslint` (`program_for`); every other name it needs (node) is looked up on PATH. Either way,
///    whatever `folder` is, its flow never runs;
/// 3. an available engine with an unresolved folder gets that folder's `Stop`;
/// 4. otherwise its entry's flow runs it through `host`.
pub(crate) fn engine_hits_with(
    rule: &Rule,
    folder: Result<&Path, &Stop>,
    host: &Host,
    versions: &[EngineVersion],
    scripts: &dyn Fn(&Path) -> Result<Vec<PathBuf>, Stop>,
) -> Result<Vec<Hit>, Stop> {
    if let Some(why) = no_file_listed(rule, folder.ok(), scripts) {
        return Err(Stop::NoFiles { why });
    }
    if rule.only_with_base && host.base.is_none() {
        return Err(Stop::NotAvailable { why: ONLY_WITH_BASE_WHY.to_string() });
    }
    if let Some(why) = list_floor(rule.engine, versions) {
        return Err(Stop::NotAvailable { why });
    }
    if let Some(stop) = unavailable(rule.engine, folder.ok(), host.on_path) {
        return Err(stop);
    }
    let folder = folder.map_err(Stop::clone)?;
    let entry = entry_of(rule.engine);
    (entry.run)(entry, rule, folder, host)
}
