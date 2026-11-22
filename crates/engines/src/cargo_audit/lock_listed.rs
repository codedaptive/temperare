//! `lock_listed`: cargo-audit's no-file outcome for a rule, its folder's Cargo.lock looked for through a given probe.

use crate::no_script::no_script;
use crate::types::Stop;
use rules::{Engine, Rule};
use std::path::{Path, PathBuf};

/// For an `Engine::CargoAudit` rule with a resolved `folder`: the one file cargo-audit reads, `Cargo.lock`, listed
/// when `has_lock(folder)` says it is there, and `no_script`'s `NoFiles` when the rule applies to no listed file. So a
/// folder without Cargo.lock, or a rule whose paths do not cover it (every finding is on Cargo.lock), has nothing to
/// check, never clean. Any other engine, or no `folder`, gives `None` without calling `has_lock`.
pub(crate) fn lock_listed(rule: &Rule, folder: Option<&Path>, has_lock: &dyn Fn(&Path) -> bool) -> Option<Stop> {
    let (Engine::CargoAudit, Some(folder)) = (rule.engine, folder) else {
        return None;
    };
    let files = if has_lock(folder) { vec![PathBuf::from("Cargo.lock")] } else { Vec::new() };
    no_script(rule, &files)
}
