//! `engine_hits`: one rule's hits from its engine's adapter, once the engine is known to be available.

use crate::ast_grep::hits_ast_grep;
use crate::availability::availability_from;
use crate::clippy::hits_clippy;
use crate::host::Host;
use crate::ruff::hits_ruff;
use crate::swiftlint::hits_swiftlint;
use crate::tessera::hits_tessera;
use crate::types::{Availability, Hit, Stop};
use rules::{Engine, Rule};
use std::path::Path;

/// The hits of `rule` under `folder`: the rule file's canonical folder, or the `Stop` its resolving gave. An engine
/// that `availability` says is not available is `NotAvailable` with that same why, whatever `folder` is, and nothing
/// runs; an available engine with an unresolved folder gets that folder's `Stop`; otherwise the rule's engine adapter
/// runs it through `host`.
pub(crate) fn engine_hits(rule: &Rule, folder: Result<&Path, &Stop>, host: &Host) -> Result<Vec<Hit>, Stop> {
    if let Availability::NotAvailable { why } = availability_from(rule.engine, host.on_path) {
        return Err(Stop::NotAvailable { why });
    }
    let folder = folder.map_err(Stop::clone)?;
    match rule.engine {
        Engine::Clippy => hits_clippy(rule, folder, host),
        Engine::SwiftLint => hits_swiftlint(rule, folder, host),
        Engine::Ruff => hits_ruff(rule, folder, host),
        Engine::AstGrep => hits_ast_grep(rule, folder, host),
        Engine::Tessera => hits_tessera(rule, folder, host),
    }
}
