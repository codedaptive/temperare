//! `GITLEAKS`: the gitleaks engine as an entry.

use super::{gitleaks_refusal, parse_gitleaks, run_gitleaks, unknown_rule_id};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// `gitleaks dir --no-banner --no-color --redact --report-format=sarif --report-path=- --enable-rule=<check>
/// [--config=<config>] .` run in the rule file's folder, its SARIF log on stdout read by `parse_gitleaks`. `dir` scans
/// the folder on disk, as ruff and pylint check it; `git` mode would read history, which no rule asks for.
/// `--no-color` keeps escape codes out of its log lines.
///
/// The check is a gitleaks rule id, or a comma list of them; an empty part or `ALL` is refused before running
/// (`gitleaks_refusal`), and gitleaks' "Requested rule <id> not found in rules" for a part of the check makes the
/// rule not available (`unknown_rule_id`). gitleaks takes no params: any param makes the rule `Failed` before
/// running. The folder's `.gitleaks.toml`, a `.gitleaksignore` and `gitleaks:allow` comments apply as gitleaks reads
/// them; the rule's config, when given, comes first.
///
/// The secret is kept out twice: `--redact` makes gitleaks print "REDACTED" in place of a match, and every hit's text
/// is one fixed text tm writes, never gitleaks'. `run_gitleaks` removes GITLEAKS_CONFIG and GITLEAKS_CONFIG_TOML from
/// the environment, so a caller's cannot replace the rule's config, and keeps only error lines of gitleaks' stderr.
/// Nothing gitleaks does here reaches the network. Gitleaks rules never share a run: `shares_with` has no arm for it.
pub static GITLEAKS: Entry = Entry {
    name: "gitleaks",
    engine: Engine::Gitleaks,
    program: "gitleaks",
    needs: &["gitleaks"],
    never: None,
    args: &["dir", "--no-banner", "--no-color", "--redact", "--report-format=sarif", "--report-path=-"],
    pick: Pick::Arg("--enable-rule="),
    params: Params::Args(&[]),
    config: Some("--config="),
    after: &["."],
    version: Some(VersionProbe { args: &["version"], read: version_word }),
    refuse: gitleaks_refusal,
    unknown: unknown_rule_id,
    read: parse_gitleaks,
    run: run_gitleaks,
};
