//! `MARKDOWNLINT`: the markdownlint engine (markdownlint-cli2) as an entry.

use super::{markdownlint_refusal, markdownlint_version, parse_markdownlint, run_markdownlint};
use crate::entry::{Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// `markdownlint-cli2 --no-globs --config <tm.markdownlint-cli2.cjs> :<file>...` run in the rule file's folder by its
/// own flow, `run_markdownlint` (over `run_markdownlint_with`), on the rule's Markdown files (`.md`, `.markdown`) git
/// lists, each passed as a literal path; the options' `globs` are dropped. Measured on markdownlint-cli2 0.24.2
/// (markdownlint 0.42.2).
///
/// Output: markdownlint-cli2 ships no JSON formatter, so tm's options module (`tm_options`, written in a temporary
/// folder and removed after the run) supplies one that prints the results as one JSON array on stdout, read by
/// `parse_markdownlint`: rule names, description, detail and context become the finding's text, as the default
/// formatter words them. It exits 0 when clean, 1 with findings and 2 for a usage or config error (a config that will
/// not load: its banner on stdout, its own message on stderr), so the JSON decides, never the exit code: stdout that
/// is not that array is `Failed`, with markdownlint-cli2's stderr when it wrote any.
///
/// The check is one markdownlint rule, by its id or an alias (`MD013`, `line-length`), matched ignoring case as
/// markdownlint does; a finding is the rule's when its `ruleNames` hold the check. A check that is not a rule name's
/// form is `Failed` before anything runs (`markdownlint_refusal`); one of that form that is not in temperare's list
/// (S-51, `MD_RULES`, taken from markdownlint 0.42.2) is not available, the engine and version named (`config_stop`),
/// since markdownlint never reports a rule it lacks. A tag (`headings`) is not a check.
///
/// Config: the repository's own. markdownlint-cli2 reads `.markdownlint.{jsonc,json,yaml,yml,cjs,mjs}` (the first in
/// that order, per folder; the nearest folder's applies) in the rule file's folder and below, never above it or in
/// HOME. tm reads the configs that apply to the rule's files (`applying_configs`): a rule its config turns off
/// (`default: false`, its name, alias or tag set falsy, or `enabled: false`) is not available, the file and key named
/// (`check_off_why`), never clean. `.json` is read by `json_settings`; `.yaml` and `.yml` by `yaml_settings` over
/// `crate::yaml` (S-78), measured against js-yaml 5.5.0, the reader markdownlint-cli2 0.24.2 bundles (YAML 1.2 core:
/// `yes`, `no`, `on`, `off` are strings, so truthy; duplicate keys are an error). A quoted scalar whose text is a
/// falsy spelling is `Failed`, since `crate::yaml` drops tags and `"false"` cannot be told from `!!bool false`. The
/// zero spellings of the YAML 1.2 core schema were measured on js-yaml 5.5.0. Stated limits, each `Failed`: a
/// `.jsonc`, `.cjs` or `.mjs` config (no safe reader), an `extends` key, a duplicate key, a `.yaml` whose text
/// markdownlint-cli2 may read as JSONC or TOML first, and a rule `config` (the entry takes none).
///
/// Suppressions (S-67 (1)): inline `<!-- markdownlint-disable* -->` and `<!-- markdownlint-configure-file -->`
/// comments are off (`noInlineConfig` in tm's module). An options file `.markdownlint-cli2.{jsonc,yaml,cjs,mjs}`
/// would override tm's module (`fix`, `ignores`, `gitignore`, `noInlineConfig`, `outputFormatters`), so one in the
/// rule file's folder or between it and a checked file makes the rule `Failed` (`options_file_why`), naming the file.
/// `.markdownlintignore` is not read by markdownlint-cli2.
///
/// Settling order: a rule whose paths hold no Markdown file git lists is `NoFiles` before availability (`engine_listed`,
/// in `engine_hits_with`'s step 1); then availability (markdownlint-cli2 on PATH); then the flow. The version is read
/// from `--help`'s first line, which lints nothing and reads no repository options (`--version` is no flag to it: it
/// would lint); `--help` exits 2, and `markdownlint_version` reads it whatever the exit code. Environment: PATH only;
/// tm sets no NODE_PATH, and markdownlint-cli2 makes no network call. markdownlint takes no params: any param makes
/// the rule `Failed` before running. Its rules never share a run: `shares_with` has no arm for it.
pub static MARKDOWNLINT: Entry = Entry {
    name: "markdownlint",
    engine: Engine::Markdownlint,
    program: "markdownlint-cli2",
    needs: &["markdownlint-cli2"],
    never: None,
    args: &["--no-globs"],
    pick: Pick::None,
    params: Params::Args(&[]),
    config: None,
    after: &[],
    version: Some(VersionProbe { args: &["--help"], read: markdownlint_version }),
    refuse: markdownlint_refusal,
    unknown: |_, _| None,
    read: parse_markdownlint,
    run: run_markdownlint,
};
