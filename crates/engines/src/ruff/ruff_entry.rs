//! `RUFF`: the ruff engine as an entry.

use super::{parse_ruff, ruff_refusal, unknown_selector};
use crate::entry::{run_plain, version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// `ruff check --no-cache --output-format json --select=<check> [--config=<config>] .` run in the rule file's folder,
/// read by `parse_ruff`. `--no-cache` leaves no `.ruff_cache/` in the checked folder (settled S-21). The check and the
/// config are joined to their options with `=`, so a value beginning with `-` is read as the value, never as an option
/// (S-35). A check tm cannot match back to its findings is refused before running (`ruff_refusal`), and ruff saying
/// it does not know a selector (`unknown_selector`) makes the rule not available. Ruff takes no params: a rule with
/// any is `Failed` before running (`param_args`).
pub static RUFF: Entry = Entry {
    name: "ruff",
    engine: Engine::Ruff,
    program: "ruff",
    needs: &["ruff"],
    never: None,
    args: &["check", "--no-cache", "--output-format", "json"],
    pick: Pick::Arg("--select="),
    params: Params::Args(&[]),
    config: Some("--config="),
    after: &["."],
    version: Some(VersionProbe { args: &["--version"], read: version_word }),
    refuse: ruff_refusal,
    unknown: unknown_selector,
    read: parse_ruff,
    run: run_plain,
};
