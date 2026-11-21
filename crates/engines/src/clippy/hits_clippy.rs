//! `hits_clippy`: one clippy rule run on the rule file's folder.

use super::{
    cargo_root, known_lint, lint_group, lint_groups_cmd, locate_workspace_cmd, parse_lint_groups, parse_lints,
    parse_locate_workspace,
};
use crate::check_run::check_run;
use crate::entry::{entry_cmd, param_args, param_why, threshold_run, Entry};
use crate::host::Host;
use crate::parse_failed::parse_failed;
use crate::ran::ran;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::ffi::OsStr;
use std::path::Path;

/// The `rule.check` hits under `folder` (canonical), with the lints `rule.params` names under `allow` turned off and
/// its `max_nesting` as the nesting threshold; `entry` is `CLIPPY`. In order, each step stopping the rule when it fails:
/// 1. The params (`param_args`, then `threshold_run`): a param other than `allow` and `max_nesting`, an `allow` that
///    is not a list of strings, or a `max_nesting` that is not a whole number from 1 to 1000 for check
///    `clippy::excessive_nesting`, is `Failed` with `param_why`'s text and nothing is run; a threshold folder that
///    cannot be written is `Failed` with `threshold_run`'s text.
/// 2. The Cargo root, the nearest folder at or above `folder` holding a `Cargo.toml`: none is `NotAvailable`.
/// 3. `clippy-driver -W help` (`lint_groups_cmd`) run in the Cargo root: it is a rustup proxy, as `cargo` is, so it
///    uses the toolchain the `cargo clippy` below uses, and lints and groups added by a newer toolchain are in that
///    toolchain's own help. `clippy-driver` absent is `NotOnPath`; help that cannot be run is `Failed`.
/// 4. The lint groups from that help: none listed is `Failed`, so the rule never reads clean. A check naming a group
///    (`lint_group`) is `Failed` and nothing more runs: a group cannot be matched by code, so the rule must name each
///    lint, or `warnings`.
/// 5. The lints from that help: none listed is `Failed`. A check that is not a listed lint (`known_lint`), unknown to
///    this toolchain or renamed, is `UnknownCheck` before clippy runs, so the rule is not available, as for a
///    toolchain too old for it. A `rustdoc::` lint never reaches here: `CLIPPY.refuse` refuses it before any flow.
///    Stopping here means a rule's own `allow` of `unknown_lints` or `renamed_and_removed_lints` cannot hide the
///    check; `parse_clippy` refusing a check clippy did not take is a second guard.
/// 6. `cargo locate-project --workspace`, then the entry's command (`entry_cmd`), both in the Cargo root: only that
///    crate and what it depends on are built and that crate is linted. Its span paths are relative to the workspace
///    root that `locate-project` names, so the entry's reader reads them from there. No `CARGO_TARGET_DIR` is set:
///    cargo builds in the repository's own `target/` (settled S-21). The clippy run is the counted check run
///    (`check_run`); the two runs before it are not. With a `max_nesting`, the check run's environment also holds
///    `CLIPPY_CONF_DIR`, naming the temporary folder whose `clippy.toml` holds the threshold; the folder is removed
///    right after the run, whatever its outcome.
pub(crate) fn hits_clippy(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let mut args = param_args(entry, &rule.params)
        .map_err(|err| Stop::Failed { code: None, stderr: param_why(entry.name, err) })?;
    let threshold = threshold_run(entry, rule, folder).map_err(|stderr| Stop::Failed { code: None, stderr })?;
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
        return Err(Stop::UnknownCheck {
            said: "it is not a lint this toolchain lists (clippy-driver -W help)".to_string(),
        });
    }
    let located = ran(&locate_workspace_cmd(&root), host)?;
    let workspace = parse_locate_workspace(&located).map_err(|err| parse_failed(&located, err))?;
    args.extend(threshold.iter().flat_map(|threshold| threshold.args.iter().cloned()));
    let mut cmd = entry_cmd(entry, OsStr::new(&rule.check), &args, None, &root);
    cmd.env.extend(threshold.iter().flat_map(|threshold| threshold.env.iter().cloned()));
    let linted = check_run(&cmd, host);
    drop(threshold);
    let out = linted?;
    (entry.read)(&out, &rule.check, &workspace, folder).map_err(|err| parse_failed(&out, err))
}
