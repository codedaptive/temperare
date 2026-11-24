//! `RUBOCOP`: the rubocop engine as an entry.

use super::{parse_rubocop, rubocop_refusal, run_rubocop, unrecognized_cop};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// `rubocop --cache false --no-server --format json --ignore-disable-comments --only=<check> [--config=<config>] .`
/// run in the rule file's folder, read by `parse_rubocop`; `run_rubocop` runs it, then the probes below.
///
/// The check is a comma-separated list of full cop names (`Lint/UselessAssignment,Style/StringLiterals`), passed as
/// `--only=`, which runs those cops even where the config disables them. Anything else (a department, a wildcard, an
/// option) is refused before running (`rubocop_refusal`). A cop rubocop does not know makes it exit 2 with
/// `Unrecognized cop or department: <cop>.` on stderr; `unrecognized_cop` reads that line, so the rule is not
/// available, never clean. Exit 0 (no offenses) and 1 (offenses) are answers; exit 2 (an unknown cop, a config it
/// cannot load, a crash) never is, even with a report on stdout. A file rubocop cannot parse makes the rule `Failed`,
/// unless the check names `Lint/Syntax`, the cop rubocop reports it under.
///
/// The config is the repository's own: the `.rubocop.yml` rubocop finds from the folder up, or the rule's `config`,
/// given as `--config=<config>`.
///
/// The environment: every command the flow runs (the check run and its probes) gets RUBOCOP_OPTS set empty, so no
/// options come from the caller's environment, and http_proxy, https_proxy, HTTP_PROXY and HTTPS_PROXY set to a dead
/// loopback proxy with no_proxy and NO_PROXY empty, so a remote `inherit_from: https://...` fails (exit 2, `Failed`)
/// instead of being fetched (`with_rubocop_env`). The version probe, `rubocop --force-default-config --version`, runs
/// without them: plain `--version` loads the folder's config first (and so would fetch a remote `inherit_from`), while
/// `--force-default-config` makes rubocop read no config file, so the probe makes no fetch. `--cache false` keeps the
/// check run from writing rubocop's result cache and `--no-server` from starting or using its server.
///
/// Suppressions, and what is done about each (S-67 (1): each is turned off, or makes the rule `Failed` with a why
/// that says exceptions go in the rule file):
/// - `# rubocop:disable` and `# rubocop:todo` comments: turned off by `--ignore-disable-comments`.
/// - a `.rubocop_todo.yml` the config reads that names a cop of the check: `Failed`, naming the file and line.
/// - a cop's `Exclude` in the config beyond rubocop's stock one: `Failed`, naming the cop and the entry. An Exclude
///   equal to the stock one is stock scope.
/// - an `AllCops` `Exclude` or `Include` that leaves out a file in the rule's paths which the stock config inspects:
///   `Failed`, naming the file.
/// - a `.rubocop` file in the folder, which rubocop reads as extra command-line options: `Failed` before any run.
///
/// Limits: two cases the probes do not see. A `.rubocop.yml` in a subfolder applies its own todo file and Excludes
/// to the files under it, but `rubocop --show-cops --debug` prints the config of the rule's folder only. A cop's
/// `Include` in the config can narrow it to fewer files than the stock one, but only `Exclude` lists are compared
/// with the stock config.
///
/// No file to check is settled inside the flow, after rubocop is found available, from the files the check run
/// says it inspected (`NoFiles` when none is in the rule's paths): rubocop's config
/// (`AllCops` `Include` and `Exclude`, from every `.rubocop.yml` up the tree) decides which files it inspects, so no
/// list of extensions made before the run can say. So, departing from settling no-files before availability: with
/// rubocop absent, a rule whose paths hold no Ruby file is not available rather than no-files, and the no-file rules
/// list does not name it. A rule whose paths match no file at all is still no-files before availability.
///
/// rubocop takes no params: any param makes the rule `Failed` before running. rubocop rules never share a run:
/// `shares_with` has no arm for it.
pub static RUBOCOP: Entry = Entry {
    name: "rubocop",
    engine: Engine::Rubocop,
    program: "rubocop",
    needs: &["rubocop"],
    never: None,
    args: &["--cache", "false", "--no-server", "--format", "json", "--ignore-disable-comments"],
    pick: Pick::Arg("--only="),
    params: Params::Args(&[]),
    config: Some("--config="),
    after: &["."],
    version: Some(VersionProbe { args: &["--force-default-config", "--version"], read: version_word }),
    refuse: rubocop_refusal,
    unknown: unrecognized_cop,
    read: parse_rubocop,
    run: run_rubocop,
};
