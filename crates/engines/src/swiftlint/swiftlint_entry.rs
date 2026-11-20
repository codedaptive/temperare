//! `SWIFTLINT`: the swiftlint engine as an entry.

use super::{parse_swiftlint, swiftlint_refusal, unknown_rule};
use crate::entry::{run_plain, version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// `swiftlint lint --reporter json --quiet --only-rule=<check> [--config=<config>]` run in the rule file's folder, so
/// swiftlint lints that folder, read by `parse_swiftlint`. The check and the config are joined to their options with
/// `=`, so a value beginning with `-` is read as the value, never as an option (S-35). A `custom_rules` check is
/// refused before running (`swiftlint_refusal`), and swiftlint warning that the id is not a valid rule identifier
/// (`unknown_rule`) makes the rule not available. Swiftlint takes no params: a rule with any is `Failed` before
/// running (`param_args`).
pub static SWIFTLINT: Entry = Entry {
    name: "swiftlint",
    engine: Engine::SwiftLint,
    program: "swiftlint",
    needs: &["swiftlint"],
    never: None,
    args: &["lint", "--reporter", "json", "--quiet"],
    pick: Pick::Arg("--only-rule="),
    params: Params::Args(&[]),
    config: Some("--config="),
    after: &[],
    version: Some(VersionProbe { args: &["version"], read: version_word }),
    refuse: swiftlint_refusal,
    unknown: unknown_rule,
    read: parse_swiftlint,
    run: run_plain,
};
