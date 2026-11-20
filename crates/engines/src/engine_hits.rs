//! `engine_hits`: one rule's hits from its engine's entry, once the engine is known to be available.

use crate::availability::availability_from;
use crate::entry_for::entry_for;
use crate::host::Host;
use crate::no_adapter_why::no_adapter_why;
use crate::types::{Availability, Hit, Stop};
use rules::Rule;
use std::path::Path;

/// The hits of `rule` under `folder`: the rule file's canonical folder, or the `Stop` its resolving gave. An engine
/// that `availability` says is not available is `NotAvailable` with that same why, whatever `folder` is, and nothing
/// runs (this covers an engine with no entry in this build, why `no_adapter_why`); an available engine with an
/// unresolved folder gets that folder's `Stop`; otherwise the flow of the rule's engine's entry (`entry_for`) runs it
/// through `host`.
pub(crate) fn engine_hits(rule: &Rule, folder: Result<&Path, &Stop>, host: &Host) -> Result<Vec<Hit>, Stop> {
    if let Availability::NotAvailable { why } = availability_from(rule.engine, host.on_path) {
        return Err(Stop::NotAvailable { why });
    }
    let folder = folder.map_err(Stop::clone)?;
    let Some(entry) = entry_for(rule.engine) else {
        return Err(Stop::NotAvailable { why: no_adapter_why(rule.engine) });
    };
    (entry.run)(entry, rule, folder, host)
}
