//! `CLIPPY`: the clippy engine as an entry.

use super::{clippy_refusal, hits_clippy, parse_clippy};
use crate::entry::{version_word, Entry, ParamArg, Params, Pick, Reach, Threshold, VersionProbe};
use rules::Engine;

/// `cargo clippy --all-targets --message-format=json --quiet -- --warn=<check> [--allow=<lint>]...` run in the Cargo
/// root, read by `parse_clippy` from the workspace root; the flow is `hits_clippy`. `--all-targets` lints tests,
/// examples and benches too: a rule's scope is its paths, not cargo's choice of targets, and without it test code is
/// never linted and reads as clean. `--warn` turns the check on as a warning so it is reported even when it is allowed
/// by default; each `--allow`, one per item of the rule's `allow` param, comes after it because the later flag wins,
/// so an allowed lint stays off even when the check (`warnings`, say) covers it. Each lint is joined to its flag with
/// `=` so a value beginning with `-` is read as the lint name, never as an option. Clippy is available only when PATH
/// holds `cargo-clippy`, which `cargo clippy` runs, and `clippy-driver`, which the flow runs for `-W help`. A
/// `rustdoc::` check is refused (`clippy_refusal`) before any flow: clippy never runs rustdoc lints.
///
/// The `max_nesting` param, read only with check `clippy::excessive_nesting` (a `-` in the rule's check reads as `_`),
/// is a whole number from 1 to 1000. tm writes `excessive-nesting-threshold = <n>` as `clippy.toml` into a temporary
/// folder named in `CLIPPY_CONF_DIR` for that run, and removes it after. With it set, clippy reads that file instead
/// of the repository's own `clippy.toml` for the run, which changes nothing for `excessive_nesting`, whose only
/// setting is the threshold. A changed `CLIPPY_CONF_DIR` makes cargo lint again, so a cached run is never replayed
/// (verified on clippy 0.1.95).
pub static CLIPPY: Entry = Entry {
    name: "clippy",
    engine: Engine::Clippy,
    program: "cargo",
    needs: &["cargo-clippy", "clippy-driver"],
    never: None,
    args: &["clippy", "--all-targets", "--message-format=json", "--quiet", "--"],
    pick: Pick::Arg("--warn="),
    params: Params::Thresholds {
        args: &[ParamArg { name: "allow", flag: "--allow=" }],
        thresholds: &[Threshold {
            name: "max_nesting",
            check: "clippy::excessive_nesting",
            max: 1000,
            setting: |n| format!("excessive-nesting-threshold = {n}\n"),
        }],
        reach: Reach::Dir { env: "CLIPPY_CONF_DIR", file: "clippy.toml" },
    },
    config: None,
    after: &[],
    version: Some(VersionProbe { args: &["clippy", "--version"], read: version_word }),
    refuse: clippy_refusal,
    unknown: |_, _| None,
    read: parse_clippy,
    run: hits_clippy,
};
