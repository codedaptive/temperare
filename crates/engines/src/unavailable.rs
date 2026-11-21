//! `unavailable`: why an engine cannot be run here, as the `Stop` its rules get.

use crate::entry_for::entry_for;
use crate::no_adapter_why::no_adapter_why;
use crate::types::Stop;
use rules::Engine;

/// The `Stop` of a rule on `engine` when the engine cannot be run here, from the PATH lookup `on_path` (nothing is
/// run), read from the engine's entry; None when it can be run. An engine with no entry in this build is
/// `NotAvailable` with `no_adapter_why`; an entry that is never available (`never`, Tessera) is `NotAvailable` with
/// that why and nothing is looked up; otherwise each name the entry `needs` is looked up in order, and the first not
/// found is `NotOnPath` naming it.
pub(crate) fn unavailable(engine: Engine, on_path: impl Fn(&str) -> bool) -> Option<Stop> {
    let Some(entry) = entry_for(engine) else {
        return Some(Stop::NotAvailable { why: no_adapter_why(engine) });
    };
    if let Some(why) = entry.never {
        return Some(Stop::NotAvailable { why: why.to_string() });
    }
    let missing = entry.needs.iter().find(|name| !on_path(name))?;
    Some(Stop::NotOnPath { program: missing.to_string() })
}
