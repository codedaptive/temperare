//! `ESLINT`: the ESLint engine as an entry.

use super::{eslint_refusal, eslint_unknown, eslint_version, parse_eslint, run_eslint};
use crate::entry::{Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// `eslint --format json --no-inline-config --no-ignore --pass-on-unpruned-suppressions
/// --no-error-on-unmatched-pattern [--config=<config>] --rule {"<check>":"error"} .` run in the rule file's folder,
/// read by `parse_eslint`; `run_eslint_with` adds the `--rule` pair.
///
/// The check is one ESLint rule id (`no-var`, `@typescript-eslint/no-explicit-any`), passed as a severity-only
/// `--rule`, so the options the config gives that rule are kept; findings are matched by their `ruleId`. A check that
/// is not a rule id is refused before running (`eslint_refusal`). A rule id the loaded config does not define (a
/// plugin it does not load, a rule its plugin lacks) is told apart by ESLint's own words on stderr (`eslint_unknown`):
/// the rule is not available (S-70). A config that does not load (an import that fails) is `Failed` with ESLint's own
/// message. Exit codes: 0 no error, 1 an error reported (an answer), 2 a config error or crash, with no JSON (`Failed`).
///
/// Suppressions: `eslint-disable` and other inline config comments are off (`--no-inline-config`), so
/// `--report-unused-disable-directives` would have nothing to report and is not passed. The config's `ignores` are
/// off (`--no-ignore`); ESLint's defaults still leave out `node_modules`, and a hit in a file git ignores is dropped
/// by the flow. Findings `eslint-suppressions.json` hides are read from `suppressedMessages`, with
/// `--pass-on-unpruned-suppressions` so its stale entries do not fail the run. ESLint 10 does not read
/// `.eslintignore`.
///
/// The eslint program is `<rule file's folder>/node_modules/.bin/eslint` when that is a file, else PATH's
/// (`program_for`), for availability, the version and the run alike, and is run by that absolute path; `node`, which
/// it needs, is looked up on PATH. A rule whose paths match no file git lists is settled before the run, as for every
/// engine (`run_rules_listed`); a run that lints no listed file in the rule's paths is settled in the flow, after
/// availability (`run_eslint_with`): ESLint's config decides which files it lints, so no list of extensions can be
/// made before the run. A fatal message (a file ESLint could not parse) is `Failed`, even in a file git does not
/// list, whose hits would be dropped (an accepted limit: the rule is never silently clean). tm sets nothing in the
/// environment (no NODE_PATH); eslint makes no network call. ESLint takes no params: any param makes the rule
/// `Failed` before running. ESLint rules never share a run: `shares_with` has no arm for it.
pub static ESLINT: Entry = Entry {
    name: "eslint",
    engine: Engine::Eslint,
    program: "eslint",
    needs: &["eslint", "node"],
    never: None,
    args: &[
        "--format",
        "json",
        "--no-inline-config",
        "--no-ignore",
        "--pass-on-unpruned-suppressions",
        "--no-error-on-unmatched-pattern",
    ],
    pick: Pick::None,
    params: Params::Args(&[]),
    config: Some("--config="),
    after: &["."],
    version: Some(VersionProbe { args: &["--version"], read: eslint_version, wrap: None }),
    refuse: eslint_refusal,
    unknown: eslint_unknown,
    read: parse_eslint,
    run: run_eslint,
};
