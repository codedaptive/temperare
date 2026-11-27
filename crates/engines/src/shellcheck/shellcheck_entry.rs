//! `SHELLCHECK`: the shellcheck engine as an entry.

use super::{parse_shellcheck, run_shellcheck, shellcheck_refusal, shellcheck_version};
use crate::entry::{Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// shellcheck run by its own flow, `run_shellcheck`: it lists the scripts and builds its command with
/// `shellcheck_cmd`, which reads none of `args`, `after` or `config`; `entry_cmd` is not used. The check is never passed (`Pick::None`): every check runs (`--enable=all`) and the reader
/// (`parse_shellcheck`) keeps the rule's code. A malformed code is refused before anything runs
/// (`shellcheck_refusal`). shellcheck never says it does not know a code, so `unknown` gives `None`; a well-formed
/// code not on cctao's list (`shellcheck_codes.txt`) is not available through the run instead. A shellcheck rule
/// takes no params.
pub static SHELLCHECK: Entry = Entry {
    name: "shellcheck",
    engine: Engine::ShellCheck,
    program: "shellcheck",
    needs: &["shellcheck"],
    never: None,
    args: &[],
    pick: Pick::None,
    params: Params::Args(&[]),
    config: Some("--rcfile="),
    after: &[],
    version: Some(VersionProbe { args: &["--version"], read: shellcheck_version, wrap: None }),
    refuse: shellcheck_refusal,
    unknown: |_, _| None,
    read: parse_shellcheck,
    run: run_shellcheck,
};
