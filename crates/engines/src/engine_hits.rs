//! `engine_hits`: one rule's hits from its engine's entry, ShellCheck's scripts listed from the repository.

use crate::engine_hits_with::engine_hits_with;
use crate::host::Host;
use crate::shellcheck::shell_files;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// `engine_hits_with` listing ShellCheck's scripts with `shell_files` (git): a ShellCheck rule's scripts, and a
/// cargo-audit rule's Cargo.lock, are looked for before availability, so an absent engine with no file in the rule's
/// paths is `NoFiles`, not `NotAvailable` (Orc #1476: no file before not available). Then a rule with `only_with_base` and no base is `NotAvailable` with
/// `rules::ONLY_WITH_BASE_WHY` before any lookup. Otherwise the engine is looked up (no entry is `NotAvailable`), then
/// one whose program is missing is `NotOnPath`, and its flow never runs; an available engine with an unresolved folder
/// gets that folder's `Stop`; otherwise its entry's flow runs the rule through `host`.
pub(crate) fn engine_hits(rule: &Rule, folder: Result<&Path, &Stop>, host: &Host) -> Result<Vec<Hit>, Stop> {
    engine_hits_with(rule, folder, host, &shell_files)
}
