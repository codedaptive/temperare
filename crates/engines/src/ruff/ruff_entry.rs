//! `RUFF`: the ruff engine as an entry.

use super::{parse_ruff, ruff_refusal, unknown_selector};
use crate::entry::{run_plain, version_word, Entry, Params, Pick, Reach, Threshold, VersionProbe};
use rules::Engine;

/// `ruff check --no-cache --output-format json --select=<check> [--config=<config>] .` run in the rule file's folder,
/// read by `parse_ruff`. `--no-cache` leaves no `.ruff_cache/` in the checked folder. The check and the
/// config are joined to their options with `=`, so a value beginning with `-` is read as the value, never as an option
///. A check cctao cannot match back to its findings is refused before running (`ruff_refusal`), and ruff saying
/// it does not know a selector (`unknown_selector`) makes the rule not available.
///
/// Ruff reads one param, `max_complexity`: a whole number from 1 to 1000, read only with check `C901`
/// (`threshold_of`). It reaches ruff as one inline argument `--config=lint.mccabe.max-complexity=<n>`, passed after
/// the rule's `--config=<config>`; ruff layers an inline `--config` over that file and over the folder's own
/// discovered config, so the threshold wins over the same key set there, and nothing is written. A threshold out of
/// range or on another check, and any other param, make the rule `Failed` before running (`threshold_run`,
/// `param_args`).
pub static RUFF: Entry = Entry {
    name: "ruff",
    engine: Engine::Ruff,
    program: "ruff",
    needs: &["ruff"],
    never: None,
    args: &["check", "--no-cache", "--output-format", "json"],
    pick: Pick::Arg("--select="),
    params: Params::Thresholds {
        args: &[],
        thresholds: &[Threshold {
            name: "max_complexity",
            check: "C901",
            max: 1000,
            setting: |n| format!("lint.mccabe.max-complexity={n}"),
        }],
        reach: Reach::Arg("--config="),
    },
    config: Some("--config="),
    after: &["."],
    version: Some(VersionProbe { args: &["--version"], read: version_word, wrap: None }),
    refuse: ruff_refusal,
    unknown: unknown_selector,
    read: parse_ruff,
    run: run_plain,
};
