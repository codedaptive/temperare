//! `engine_hits_with`: one rule's hits from its engine's entry, with ShellCheck's no-file outcome settled before
//! availability through a given lister of scripts.

use crate::availability::availability_from;
use crate::entry_for::entry_for;
use crate::host::Host;
use crate::no_adapter_why::no_adapter_why;
use crate::shellcheck::no_script;
use crate::types::{Availability, Hit, Stop};
use rules::{Engine, Rule};
use std::path::{Path, PathBuf};

/// The hits of `rule` under `folder`: the rule file's canonical folder, or the `Stop` its resolving gave. In the
/// order every engine settles a rule (refusals, which the caller settles first; no file; not available; the run):
/// 1. a ShellCheck rule with a resolved folder lists its scripts with `scripts`; when `no_script` finds none the rule
///    applies to, it is `NoFiles`, nothing is looked up on PATH and the flow never runs. A listing error falls
///    through to the steps below (an available ShellCheck's flow lists again and stops with it);
/// 2. an engine that `availability` says is not available is `NotAvailable` with that same why, whatever `folder`
///    is, and its flow never runs (this covers an engine with no entry in this build, why `no_adapter_why`);
/// 3. an available engine with an unresolved folder gets that folder's `Stop`;
/// 4. otherwise the flow of the rule's engine's entry (`entry_for`) runs it through `host`.
pub(crate) fn engine_hits_with(
    rule: &Rule,
    folder: Result<&Path, &Stop>,
    host: &Host,
    scripts: &dyn Fn(&Path) -> Result<Vec<PathBuf>, Stop>,
) -> Result<Vec<Hit>, Stop> {
    if let (Engine::ShellCheck, Ok(folder)) = (rule.engine, folder) {
        if let Some(stop) = scripts(folder).ok().and_then(|listed| no_script(rule, &listed)) {
            return Err(stop);
        }
    }
    if let Availability::NotAvailable { why } = availability_from(rule.engine, host.on_path) {
        return Err(Stop::NotAvailable { why });
    }
    let folder = folder.map_err(Stop::clone)?;
    let Some(entry) = entry_for(rule.engine) else {
        return Err(Stop::NotAvailable { why: no_adapter_why(rule.engine) });
    };
    (entry.run)(entry, rule, folder, host)
}
