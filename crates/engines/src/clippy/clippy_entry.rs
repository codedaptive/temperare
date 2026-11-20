//! `CLIPPY`: the clippy engine as an entry.

use super::{hits_clippy, parse_clippy};
use crate::entry::{version_word, Entry, ParamArg, Params, Pick, VersionProbe};
use rules::Engine;

/// `cargo clippy --all-targets --message-format=json --quiet -- --warn=<check> [--allow=<lint>]...` run in the Cargo
/// root, read by `parse_clippy` from the workspace root; the flow is `hits_clippy`. `--all-targets` lints tests,
/// examples and benches too: a rule's scope is its paths, not cargo's choice of targets, and without it test code is
/// never linted and reads as clean. `--warn` turns the check on as a warning so it is reported even when it is allowed
/// by default; each `--allow`, one per item of the rule's `allow` param, comes after it because the later flag wins,
/// so an allowed lint stays off even when the check (`warnings`, say) covers it. Each lint is joined to its flag with
/// `=` so a value beginning with `-` is read as the lint name, never as an option. Clippy is available only when PATH
/// holds `cargo-clippy`, which `cargo clippy` runs, and `clippy-driver`, which the flow runs for `-W help`.
pub static CLIPPY: Entry = Entry {
    name: "clippy",
    engine: Engine::Clippy,
    program: "cargo",
    needs: &["cargo-clippy", "clippy-driver"],
    never: None,
    args: &["clippy", "--all-targets", "--message-format=json", "--quiet", "--"],
    pick: Pick::Arg("--warn="),
    params: Params::Args(&[ParamArg { name: "allow", flag: "--allow=" }]),
    config: None,
    after: &[],
    version: Some(VersionProbe { args: &["clippy", "--version"], read: version_word }),
    refuse: |_| None,
    unknown: |_, _| None,
    read: parse_clippy,
    run: hits_clippy,
};
