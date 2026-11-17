//! `hits_clippy`: one clippy rule run on the rule file's folder.

use super::{cargo_root, clippy_cmd, locate_workspace_cmd, parse_clippy, parse_locate_workspace};
use crate::host::Host;
use crate::parse_failed::parse_failed;
use crate::ran::ran;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// The `rule.check` lint's hits under `folder` (canonical). Clippy runs in the nearest folder holding a `Cargo.toml`,
/// so only that crate and what it depends on are built and that crate is linted; its span paths are relative to the
/// workspace root, which `cargo locate-project --workspace` names, so they are read from there. No `CARGO_TARGET_DIR`
/// is set: cargo builds in the repository's own `target/` (settled S-21).
pub(crate) fn hits_clippy(rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let root = cargo_root(folder)
        .ok_or_else(|| Stop::NotAvailable { why: "no Cargo.toml at or above the rule file's folder".to_string() })?;
    let located = ran(&locate_workspace_cmd(&root), host)?;
    let workspace = parse_locate_workspace(&located).map_err(|err| parse_failed(&located, err))?;
    let out = ran(&clippy_cmd(&rule.check, &root), host)?;
    parse_clippy(&out, &rule.check, &workspace, folder).map_err(|err| parse_failed(&out, err))
}
