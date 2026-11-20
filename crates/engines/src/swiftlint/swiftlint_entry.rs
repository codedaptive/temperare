//! `SWIFTLINT`: the swiftlint engine as an entry.

use super::{parse_swiftlint, swiftlint_refusal, unknown_rule};
use crate::entry::{run_plain, version_word, Entry, Params, Pick, Reach, Threshold, VersionProbe};
use rules::Engine;

/// `swiftlint lint --reporter json --quiet --only-rule=<check> [--config=<config>]` run in the rule file's folder, so
/// swiftlint lints that folder, read by `parse_swiftlint`. The check and the config are joined to their options with
/// `=`, so a value beginning with `-` is read as the value, never as an option (S-35). A `custom_rules` check is
/// refused before running (`swiftlint_refusal`), and swiftlint warning that the id is not a valid rule identifier
/// (`unknown_rule`) makes the rule not available.
///
/// Swiftlint reads two params (`threshold_of`), at most one per rule: `max_complexity`, a whole number from 1 to
/// 1000, read only with check `cyclomatic_complexity`, and `max_lines`, a whole number from 1 to 100000, read only
/// with check `file_length`. The threshold is written to a temporary `.yml` file setting the check's `warning` and
/// `error` levels both to `n`, passed as the last `--config=<path>` and removed after the run. Both levels are set: swiftlint
/// rejects `error: null` and falls back to its default, and an error level below `n` left by an earlier config would
/// still report; tm does not read swiftlint's severity. Swiftlint merges several `--config=`, a later one winning per
/// key, so the threshold comes after the rule's config. An explicit `--config` makes swiftlint skip the folder's own
/// `.swiftlint.yml`, so when the rule gives no config and that file is there it is passed first. A threshold out of
/// range or on another check, and any other param, make the rule `Failed` before running (`threshold_run`,
/// `param_args`).
pub static SWIFTLINT: Entry = Entry {
    name: "swiftlint",
    engine: Engine::SwiftLint,
    program: "swiftlint",
    needs: &["swiftlint"],
    never: None,
    args: &["lint", "--reporter", "json", "--quiet"],
    pick: Pick::Arg("--only-rule="),
    params: Params::Thresholds {
        args: &[],
        thresholds: &[
            Threshold {
                name: "max_complexity",
                check: "cyclomatic_complexity",
                max: 1000,
                setting: |n| format!("cyclomatic_complexity:\n  warning: {n}\n  error: {n}\n"),
            },
            Threshold {
                name: "max_lines",
                check: "file_length",
                max: 100000,
                setting: |n| format!("file_length:\n  warning: {n}\n  error: {n}\n"),
            },
        ],
        reach: Reach::File { option: "--config=", ext: "yml", own: Some(".swiftlint.yml") },
    },
    config: Some("--config="),
    after: &[],
    version: Some(VersionProbe { args: &["version"], read: version_word }),
    refuse: swiftlint_refusal,
    unknown: unknown_rule,
    read: parse_swiftlint,
    run: run_plain,
};
