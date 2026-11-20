//! `hits_clippy`: one clippy rule run on the rule file's folder.

use super::{
    cargo_root, clippy_allows, clippy_cmd, known_lint, lint_group, lint_groups_cmd, locate_workspace_cmd, parse_clippy,
    parse_lint_groups, parse_lints, parse_locate_workspace,
};
use crate::check_run::check_run;
use crate::host::Host;
use crate::parse_failed::parse_failed;
use crate::ran::ran;
use crate::types::{Hit, Stop};
use crate::ParamError;
use rules::Rule;
use std::path::Path;

/// The `rule.check` hits under `folder` (canonical), with the lints `rule.params` names under `allow` turned off.
/// In order, each step stopping the rule when it fails:
/// 1. The params: a param other than `allow`, or an `allow` that is not a list of strings, is `Failed` with the
///    reason and nothing is run.
/// 2. The Cargo root, the nearest folder at or above `folder` holding a `Cargo.toml`: none is `NotAvailable`.
/// 3. `clippy-driver -W help` (`lint_groups_cmd`) run in the Cargo root: it is a rustup proxy, as `cargo` is, so it
///    uses the toolchain the `cargo clippy` below uses, and lints and groups added by a newer toolchain are in that
///    toolchain's own help. `clippy-driver` absent is `NotAvailable`; help that cannot be run is `Failed`.
/// 4. The lint groups from that help: none listed is `Failed`, so the rule never reads clean. A check naming a group
///    (`lint_group`) is `Failed` and nothing more runs: a group cannot be matched by code, so the rule must name each
///    lint, or `warnings`.
/// 5. The lints from that help: none listed is `Failed`. A check that is not a listed lint (`known_lint`) is `Failed`
///    and nothing more runs: a `rustdoc::` lint, which clippy never runs, or an unknown or renamed name, which clippy
///    would not run as named. These are refused before clippy runs, so a rule's own `allow` of `unknown_lints` or
///    `renamed_and_removed_lints` cannot hide them; `parse_clippy` refusing a check clippy did not take is a second
///    guard.
/// 6. `cargo locate-project --workspace`, then clippy, both in the Cargo root: only that crate and what it depends on
///    are built and that crate is linted. Its span paths are relative to the workspace root that `locate-project`
///    names, so they are read from there. No `CARGO_TARGET_DIR` is set: cargo builds in the repository's own
///    `target/` (settled S-21). The clippy run is the counted check run (`check_run`); the two runs before it are not.
pub(crate) fn hits_clippy(rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let allows = clippy_allows(&rule.params).map_err(param_failed)?;
    let root = cargo_root(folder)
        .ok_or_else(|| Stop::NotAvailable { why: "no Cargo.toml at or above the rule file's folder".to_string() })?;
    let help = ran(&lint_groups_cmd(&root), host)?;
    let groups = parse_lint_groups(&help).map_err(|err| parse_failed(&help, err))?;
    if lint_group(&rule.check, &groups) {
        let stderr = format!(
            "clippy check `{}` is a lint group: tm cannot tell its lints from other warnings, so it cannot report it; name each lint, or use `warnings`",
            rule.check
        );
        return Err(Stop::Failed { code: None, stderr });
    }
    let lints = parse_lints(&help).map_err(|err| parse_failed(&help, err))?;
    if !known_lint(&rule.check, &lints) {
        return Err(Stop::Failed { code: None, stderr: unknown_why(&rule.check) });
    }
    let located = ran(&locate_workspace_cmd(&root), host)?;
    let workspace = parse_locate_workspace(&located).map_err(|err| parse_failed(&located, err))?;
    let out = check_run(&clippy_cmd(&rule.check, &allows, &root), host)?;
    parse_clippy(&out, &rule.check, &workspace, folder).map_err(|err| parse_failed(&out, err))
}

/// Why a check the toolchain does not list as a lint is refused: a `rustdoc::` lint is named as such, since it is a
/// real lint that clippy never runs; any other name is one the toolchain does not list.
fn unknown_why(check: &str) -> String {
    if check.starts_with("rustdoc::") {
        format!("clippy check `{check}` is a rustdoc lint: clippy never runs rustdoc lints, so tm cannot report it")
    } else {
        format!(
            "clippy check `{check}` is not a lint this toolchain lists (clippy-driver -W help): tm cannot tell it was run, so it cannot report it; name a lint it lists, or use `warnings`"
        )
    }
}

/// A param error as the rule's failure: no exit code, since nothing ran, and the reason in place of stderr.
fn param_failed(err: ParamError) -> Stop {
    let stderr = match err {
        ParamError::Bad { name, why } => format!("clippy param {name}: {why}"),
        ParamError::Unknown { name } => format!("clippy param {name}: unknown"),
    };
    Stop::Failed { code: None, stderr }
}
