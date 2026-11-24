//! `DETEKT`: the detekt engine as an entry.

use super::{detekt_refusal, parse_detekt, run_detekt, unknown_rule};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// `detekt --input <files> --config [<config>,]<layer> --report sarif:<report>` run in the rule file's folder
/// (`run_detekt`, `detekt_args`), the report file's SARIF log read by `parse_detekt`. The files are the Kotlin sources
/// (`.kt`, `.kts`) the rule applies to, relative to the folder: tracked, and untracked but not ignored. What git
/// ignores (build output, generated code) is never given, and detekt without `--input` would scan the whole folder.
///
/// A check is detekt rule names and rule-set names, comma-separated (`MagicNumber`, `style`), as detekt 1.23.8's
/// built-in list writes them (`detekt_rules.txt`). A malformed name is refused before the run (`detekt_refusal`); a
/// well-formed name not in the list is not available, its why saying the list missed it (`detekt_unlisted`). tm picks
/// a check's rules with its own config layer (`detekt_layer`), passed last so it wins key by key: each picked rule and
/// its set active, with no path filter. The reader keeps only the picked rules' findings, by SARIF rule id. A rule tm
/// lists that the running detekt lacks fails detekt's config validation; detekt's words naming it make the rule not
/// available (`unknown_rule`). 67 rules need type resolution (`detekt_typed_rules.txt`), which needs a classpath tm
/// does not give detekt, so without one they report nothing: a check, rule or set, that picks any of them is not
/// available with `typed_why`'s why, and nothing runs. Only the sets with no such rule (`comments`, `empty-blocks`)
/// run as sets, each turning on every rule of the set, those detekt ships inactive included: `comments` holds
/// AbsentOrWrongFileLicense, which needs a license template file, and without one detekt exits 1 and the rule is
/// `Failed`, never clean.
///
/// The config is the rule's `config`, else the folder's `detekt.yml` when there is one, else none; tm's layer comes
/// after it. It is never `--build-upon-default-config`: the repository's config and tm's layer are the whole config,
/// and an option neither sets takes detekt's in-code default. A rule's own options in the repository's config
/// (MagicNumber's `ignoreNumbers`, a threshold) are the engine's configuration and apply; what switches the rule off or
/// silences it by path or by annotation is an exception, which the layer resets.
///
/// Suppressions. The layer resets, for each picked rule, a repository config's `active: false`, `excludes`,
/// `includes` and `ignoreAnnotated`, and the set's `active`, `excludes` and `includes`. A baseline is never passed
/// (`--baseline`), and detekt reads none by itself, so a repository's `detekt-baseline.xml` hides nothing.
/// `@Suppress` in the code is honoured by detekt even under tm's layer (fixtures/commands.txt, suppress), and no
/// option tm passes turns it off: a gap the rule file cannot close.
///
/// A rule with no Kotlin source in its paths is `NoFiles` before availability is looked at (`engine_hits_with`).
/// detekt exits 0 with no issues (or no more than a config's `maxIssues`) and 2 with issues; both are answers. 3 (an
/// invalid config) and 1 (a usage error or a crash; commands.txt: usage, and AbsentOrWrongFileLicense with no template)
/// are not: the rule is `Failed`. A file with a syntax error (an unclosed brace) was still analysed in the recorded
/// run (fixtures/commands.txt). `run_detekt` removes JAVA_TOOL_OPTIONS, JDK_JAVA_OPTIONS and _JAVA_OPTIONS from the
/// environment and keeps JAVA_HOME. tm passes no `--plugins`, `--classpath` or `--baseline`, and none of the options it
/// passes asks detekt to download anything. detekt takes no params: any param makes the rule `Failed` before running.
/// detekt rules never share a run: `shares_with` has no arm for it.
pub static DETEKT: Entry = Entry {
    name: "detekt",
    engine: Engine::Detekt,
    program: "detekt",
    needs: &["detekt"],
    never: None,
    args: &[],
    pick: Pick::None,
    params: Params::Args(&[]),
    config: None,
    after: &[],
    version: Some(VersionProbe { args: &["--version"], read: version_word }),
    refuse: detekt_refusal,
    unknown: unknown_rule,
    read: parse_detekt,
    run: run_detekt,
};
