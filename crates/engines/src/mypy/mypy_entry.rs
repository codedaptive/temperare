//! `MYPY`: the mypy engine as an entry.

use super::{mypy_refusal, parse_mypy, run_mypy, unknown_code};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// mypy run by its own flow, `run_mypy`, with the command `mypy_cmd` builds:
/// `mypy --output=json --cache-dir=/dev/null --enable-error-code=<check> --config-file=<config or empty> -- <files>`,
/// run in the rule file's folder on the Python files in the rule's scope (`python_files`) and read by `parse_mypy`.
///
/// The check is one mypy error code (`mypy_refusal` refuses any other shape before anything runs). It reaches mypy as
/// `--enable-error-code`, which turns on an optional code and beats a global `disable_error_code`; the reader keeps
/// only that code's findings. A code tied to a strictness flag (`no-untyped-def` needs `disallow_untyped_defs`) is
/// reported only when the repository's config sets that flag. mypy saying "Invalid error code(s)" of the check is
/// `UnknownCheck` (`unknown_code`).
///
/// The config tm pins: the rule's config, else the one mypy itself would find from the folder up to the repository's
/// top (`mypy_config`), else none (`--config-file=`), so the user's home config is never read. Suppressions, settled
/// S-67: (1) a pinned config whose `ignore_errors`, per-module `disable_error_code` or `ignore_missing_imports`
/// silences the check makes the rule `Failed` (`config_suppression`); config `exclude` does not apply, since the files
/// are passed explicitly. (2) Left honoured: `# type: ignore[...]`, inline `# mypy: ignore-errors` and
/// `# mypy: disable-error-code=...`, and `@typing.no_type_check`.
///
/// No Python file in the rule's scope is `NoFiles`, settled before availability (`engine_listed`). Exit 0 and 1 are
/// answers; exit 2 is a blocking error after which mypy checks nothing else, so the rule is `Failed`, except for the
/// check `syntax` when mypy printed errors and every one is a syntax error: those are its findings. A duplicate
/// module or no Python file is `Failed` for `syntax` too. `--cache-dir=/dev/null` writes no cache and
/// `--install-types` is never passed, so nothing reaches the network and no environment needs setting. A mypy rule
/// takes no params and shares no run with another rule.
pub static MYPY: Entry = Entry {
    name: "mypy",
    engine: Engine::Mypy,
    program: "mypy",
    needs: &["mypy"],
    never: None,
    args: &["--output=json", "--cache-dir=/dev/null"],
    pick: Pick::Arg("--enable-error-code="),
    params: Params::Args(&[]),
    config: Some("--config-file="),
    after: &[],
    version: Some(VersionProbe { args: &["--version"], read: version_word }),
    refuse: mypy_refusal,
    unknown: unknown_code,
    read: parse_mypy,
    run: run_mypy,
};
