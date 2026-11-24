//! `STYLELINT`: the stylelint engine as an entry.

use super::{parse_stylelint, run_stylelint, stylelint_refusal, stylelint_unknown};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// `stylelint --formatter json --ignore-disables --ignore-path=/dev/null --disable-default-ignores --allow-empty-input
/// --no-cache [--config=<config>] --suppress-location=<empty file> ./<file>...` run in the rule file's folder, read by
/// `parse_stylelint` from stderr, where stylelint writes its JSON report; `run_stylelint_with` appends the
/// suppressions file and the files.
///
/// The check is one stylelint rule name (`color-no-invalid-hex`, `scss/at-rule-no-unknown`); a check that is not one is
/// refused before running (`stylelint_refusal`). stylelint has no option that passes or turns on one rule, so the
/// config decides: before the check run, each file is probed with `stylelint --print-config [--config=<config>]
/// ./<file>` (`configs_stop`). A config that turns the check on for no file makes the rule `NotAvailable`; one that
/// turns it off for some files is `Failed` naming the first, since exceptions go in the rule file; a config that does
/// not load is `Failed` with stylelint's own message. A name the config gives but stylelint does not know prints as on,
/// and the check run reports it as `Unknown rule <name>`, told apart by `stylelint_unknown`: the rule is not available
/// (S-70). Findings are matched by their `rule`.
///
/// Suppressions, each turned off or refused (S-67 (1)): `stylelint-disable` comments are off (`--ignore-disables`);
/// `.stylelintignore` is off (`--ignore-path=/dev/null`); a `stylelint-suppressions.json` is never read, since
/// stylelint is pointed at an empty one tm writes (`--suppress-location`); the config's `ignoreFiles` has no option to
/// turn it off, so a file it ignores is `Failed` naming the file (`report_why`); a config turning on `quiet`, which
/// drops warning-severity findings, is `Failed` at the probe. stylelint's default ignores (`node_modules`) are off
/// (`--disable-default-ignores`), so an ignored file is only ever the config's. A config turning on `fix` would make
/// stylelint rewrite the checked files, which no option stops, so it is `Failed` at the probe and the check never
/// runs; a rewritten file in a report is `Failed` too. `--no-cache` keeps stylelint from writing `.stylelintcache`.
///
/// Exit codes: 0 no error-severity problem and 2 a problem reported, each an answer with the report on stderr; 1 a
/// fatal error, 64 a usage error and 78 a config error, each text with no report (`Failed`). A CSS syntax error or an
/// invalid rule option is in the report, and `Failed` naming the file.
///
/// Files: tm passes the `.css` files git lists under the folder that the rule applies to (`is_css_source`, any ASCII
/// case), since `stylelint .` would lint every file there. Other syntaxes (SCSS, Less, CSS in HTML) need a
/// `customSyntax` the repository installs, so they are not checked: a stated limit. A rule with no such file is settled
/// as no file before availability (`engine_listed`). The probe costs one Node process per file, the price of never
/// letting a config rewrite a file or hide an exception for some files silently. The probes are not counted; the check
/// run is.
///
/// stylelint runs from PATH; tm sets nothing in the environment (no NODE_PATH, S-72 (3)), and stylelint makes no
/// network call. stylelint takes no params: any param makes the rule `Failed` (`refused`, before anything runs; the
/// flow's own check, after its probes, is the same refusal). stylelint rules never share a run: `shares_with` has no
/// arm for it.
pub static STYLELINT: Entry = Entry {
    name: "stylelint",
    engine: Engine::Stylelint,
    program: "stylelint",
    needs: &["stylelint"],
    never: None,
    args: &[
        "--formatter",
        "json",
        "--ignore-disables",
        "--ignore-path=/dev/null",
        "--disable-default-ignores",
        "--allow-empty-input",
        "--no-cache",
    ],
    pick: Pick::None,
    params: Params::Args(&[]),
    config: Some("--config="),
    after: &[],
    version: Some(VersionProbe { args: &["--version"], read: version_word }),
    refuse: stylelint_refusal,
    unknown: stylelint_unknown,
    read: parse_stylelint,
    run: run_stylelint,
};
