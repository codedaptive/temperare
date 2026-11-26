//! `ACTIONLINT`: the actionlint engine as an entry.

use super::{actionlint_refusal, parse_actionlint, run_actionlint, unknown_kind};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// The Go template actionlint runs once over all its errors: one JSON object holding the kinds this run defined
/// (`allKinds`) and the errors found, `[]` when clean.
const TEMPLATE: &str = r#"{"kinds":{{json allKinds}},"errors":{{json .}}}"#;

/// `actionlint -no-color -format <TEMPLATE> -config-file=<config> -shellcheck=<...> -pyflakes=<...> -- <workflows>`,
/// run in the rule file's folder by its own flow, `run_actionlint`, and read by `parse_actionlint`.
///
/// The check is one actionlint kind (`expression`, `action`, `shellcheck`, ...). No option selects a kind: every kind
/// runs and the reader keeps the rule's. A check that is not lowercase words joined by `-` is `Failed` before anything
/// runs (`actionlint_refusal`). A well-formed kind the run did not define is not available, read from the run's own
/// `allKinds` (`unknown_kind`), never clean.
///
/// The ShellCheck and pyflakes integrations are on only for their own check (`integration_args`), which then needs
/// `shellcheck` or `pyflakes` on PATH, else `NotOnPath`: actionlint given a program it cannot find drops the kind and
/// skips it silently. For any other check both are off, since the reader would drop their findings anyway and running
/// them would make the run depend on PATH. `SHELLCHECK_OPTS` is removed from the run's environment, since it can
/// silence ShellCheck findings (S-67 (1)).
///
/// The config is the one tm decides, and tm always passes it as `-config-file=`: the rule's config (folder-relative),
/// else the first of `<top>/.github/actionlint.yaml` and `<top>/.github/actionlint.yml` that is a file (`<top>` the
/// git top, as actionlint discovers it), else `/dev/null` (`actionlint_config`). So actionlint reads exactly the file
/// tm read. tm reads it through `crate::yaml`: an `ignore` key in it drops findings and tm cannot turn it off, so it is
/// `Failed` naming the file and line (S-67 (1), `config_suppression`). actionlint 1.7.12 reads YAML with
/// `go.yaml.in/yaml/v4` (v4.0.0-rc.3), measured (fixtures/fixtures.txt, "Config reading"): a duplicate key is fatal
/// (exit 3); only the first document applies; a merge key `<<` applies, so an `ignore` merged from an anchor drops
/// findings; `Ignore` (another case) is not honoured; an escaped key `"\x69gnore"` is. `yaml_entries` rejects a
/// duplicate key, a second document and `<<`, each `Failed`, never a silent clean. `self-hosted-runner` and
/// `config-variables` are the repository's own definitions, left as they are.
///
/// What still keeps a finding from the report outside the rule file: `# shellcheck disable=...` inside a `run:`
/// script, which actionlint's ShellCheck honours and no option turns off (S-67 (2)).
///
/// Only the workflows directly in `<folder>/.github/workflows` (`is_workflow`) the rule applies to are passed; with
/// none the rule is `NoFiles`, settled before availability (`engine_listed`, in `engine_hits_with`'s step 1).
/// actionlint exits 0 clean, 1 with errors, 2 for a bad flag and 3 when fatal; tm reads the JSON, not the exit code.
/// Nothing reaches the network: popular-action data is built in and local actions are read from disk. actionlint
/// takes no params, and its rules never share a run: `shares_with` has no arm for it.
pub static ACTIONLINT: Entry = Entry {
    name: "actionlint",
    engine: Engine::Actionlint,
    program: "actionlint",
    needs: &["actionlint"],
    never: None,
    args: &["-no-color", "-format", TEMPLATE],
    pick: Pick::None,
    params: Params::Args(&[]),
    config: None,
    after: &[],
    version: Some(VersionProbe { args: &["-version"], read: version_word, wrap: None }),
    refuse: actionlint_refusal,
    unknown: unknown_kind,
    read: parse_actionlint,
    run: run_actionlint,
};
