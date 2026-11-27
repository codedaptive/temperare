//! `HADOLINT`: the hadolint engine as an entry.

use super::{hadolint_refusal, parse_hadolint, run_hadolint};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// hadolint run by its own flow, `run_hadolint`, with the command `hadolint_cmd` builds:
/// `hadolint --no-color -f sarif --disable-ignore-pragma --error=<check> --config=<config> -- <files>`, run in the rule
/// file's folder and read by `parse_hadolint`, with every variable in cctao's environment whose name starts with
/// `HADOLINT_` removed from its environment.
///
/// The check is one DL code from cctao's list. It reaches hadolint as `--error=<check>`, which also turns on a rule
/// that is off by default (DL1001); the reader keeps only that code's findings. A check not shaped `DL` or `SC` then
/// digits is `Failed` before anything runs (`hadolint_refusal`). hadolint takes any text as a code and never says it
/// does not know one, so cctao carries the list of DL codes in hadolint 2.15.1 (`hadolint_codes.txt`) and a code
/// not on it is not available (`not_listed`), never run and never clean. An SC code is not available too: hadolint
/// runs ShellCheck without its optional checks and leaves out codes standalone ShellCheck reports, so cctao cannot list
/// the SC codes it emits.
///
/// The files are the rule's in-scope `Dockerfile`, `Containerfile` and `*.Dockerfile` files (`source_files` with
/// `is_dockerfile`). With none the rule is `NoFiles`, settled before availability (`engine_listed`, in
/// `engine_hits_with`'s step 1).
///
/// The config is always pinned with `--config`, which makes hadolint read that file and nothing else: the rule's
/// config, else the folder's `.hadolint.yaml`, else its `.hadolint.yml` (`hadolint_config`), else a temporary file
/// holding `{}`, so a machine's own config under `$XDG_CONFIG_HOME` or the home folder is never read.
///
/// Suppressions: (1) inline `# hadolint ignore=` and `# hadolint global ignore=` pragmas are turned off
/// (`--disable-ignore-pragma`); a pinned config whose `ignored` lists anything makes the rule `Failed`
/// (`ignored_listed`), since `--ignore` cannot take it back; every variable in cctao's environment whose name starts with
/// `HADOLINT_`, `HADOLINT_IGNORE` among them, is removed. (2) Left honoured: `# shellcheck disable=` inside a `RUN`
/// heredoc, which hadolint's ShellCheck obeys even with the pragmas off.
///
/// hadolint exits 1 when it has findings at or above its threshold; any exit with a SARIF log on stdout is an answer.
/// A config hadolint cannot read (it says so and runs on its defaults) and a Dockerfile it cannot parse (a DL1000
/// result, with that file's other findings missing) are `Failed`. Under a sandbox that denies all network, hadolint
/// 2.15.1 gave the same stdout, stderr and exit code as without it: in that probe run (one Dockerfile, one config),
/// its output did not depend on the network (measured). cctao sets nothing in the run's environment beyond removing every
/// variable whose name starts with `HADOLINT_`. A hadolint rule takes no params and shares no run with another rule:
/// `shares_with` has no arm for it.
pub static HADOLINT: Entry = Entry {
    name: "hadolint",
    engine: Engine::Hadolint,
    program: "hadolint",
    needs: &["hadolint"],
    never: None,
    args: &["--no-color", "-f", "sarif", "--disable-ignore-pragma"],
    pick: Pick::Arg("--error="),
    params: Params::Args(&[]),
    config: None,
    after: &[],
    version: Some(VersionProbe { args: &["--version"], read: version_word, wrap: None }),
    refuse: hadolint_refusal,
    unknown: |_, _| None,
    read: parse_hadolint,
    run: run_hadolint,
};
