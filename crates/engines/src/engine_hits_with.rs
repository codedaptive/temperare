//! `engine_hits_with`: one rule's hits from its engine's entry, with ShellCheck's no-file outcome (through a given
//! lister of scripts) and cargo-audit's settled before availability, and a rule that needs a base settled without one.

use crate::cargo_audit::{has_lock, lock_listed};
use crate::entry_for::entry_for;
use crate::host::Host;
use crate::no_adapter_why::no_adapter_why;
use crate::shellcheck::no_script_listed;
use crate::types::{Hit, Stop};
use crate::unavailable::unavailable;
use rules::{Rule, ONLY_WITH_BASE_WHY};
use std::path::{Path, PathBuf};

/// The hits of `rule` under `folder`: the rule file's canonical folder, or the `Stop` its resolving gave. In the
/// order every engine settles a rule (refusals, which the caller settles first; no file; not available; the run):
/// 1. a ShellCheck rule with a resolved folder lists its scripts with `scripts`; when `no_script_listed` finds none
///    the rule applies to, it is `NoFiles`, nothing is looked up on PATH and the flow never runs. A listing error
///    falls through to the steps below (an available ShellCheck's flow lists again and stops with it). Then a
///    cargo-audit rule with a resolved folder is `NoFiles` when `lock_listed` finds no Cargo.lock there (`has_lock`
///    reads the disk) or the rule's paths do not cover it; likewise nothing is looked up and the flow never runs;
/// 1b. a rule with `only_with_base` and no `host.base` is `NotAvailable` with `rules::ONLY_WITH_BASE_WHY`, whatever its
///    engine and whatever `folder` is: nothing is looked up on PATH and the flow never runs;
/// 2. an engine with no entry in this build (`entry_for`) is `NotAvailable` with `no_adapter_why`; an engine that
///    `unavailable` says cannot be run here gets that stop: `NotOnPath` for a program it needs not on PATH,
///    `NotAvailable` for one never available. Either way, whatever `folder` is, its flow never runs;
/// 3. an available engine with an unresolved folder gets that folder's `Stop`;
/// 4. otherwise its entry's flow runs it through `host`.
pub(crate) fn engine_hits_with(
    rule: &Rule,
    folder: Result<&Path, &Stop>,
    host: &Host,
    scripts: &dyn Fn(&Path) -> Result<Vec<PathBuf>, Stop>,
) -> Result<Vec<Hit>, Stop> {
    if let Some(stop) = no_script_listed(rule, folder.ok(), scripts) {
        return Err(stop);
    }
    if let Some(stop) = lock_listed(rule, folder.ok(), &has_lock) {
        return Err(stop);
    }
    if rule.only_with_base && host.base.is_none() {
        return Err(Stop::NotAvailable { why: ONLY_WITH_BASE_WHY.to_string() });
    }
    let Some(entry) = entry_for(rule.engine) else {
        return Err(Stop::NotAvailable { why: no_adapter_why(rule.engine) });
    };
    if let Some(stop) = unavailable(rule.engine, host.on_path) {
        return Err(stop);
    }
    let folder = folder.map_err(Stop::clone)?;
    (entry.run)(entry, rule, folder, host)
}
