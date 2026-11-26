//! `DART`: the Dart analyzer (`dart analyze`) as an entry.

use super::{dart_refusal, parse_dart, run_dart};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// `dart --suppress-analytics analyze --format machine .` run in the rule file's folder, read by `parse_dart`. The
/// analyzer checks the Dart package the folder is in, as the package's own `analysis_options.yaml` sets it up; a rule's
/// config never reaches dart. Findings outside the rule's scope are dropped later.
///
/// The check is one diagnostic code or lint name, lower case (`unused_local_variable`, `prefer_single_quotes`). Dart
/// has no option that selects a diagnostic: it reports every one it finds, and the reader keeps those whose code is
/// the check. A check of any other shape is refused before running (`dart_refusal`).
///
/// Before the check run, `run_dart` runs a probe: `dart analyze` in a new temporary package whose options set the
/// check as an error code and enable it as a lint (`probe_files`, `ProbeDir`), not counted. Dart warns on a code it
/// does not know at all (an unknown check, not available, never clean) and on one that is not a lint (a diagnostic,
/// reported wherever dart finds it) (`kind_of`). A lint runs only where analysis_options.yaml enables it, so a lint
/// that is not enabled in every options file governing the folder is `NotAvailable` (`lint_enabled`).
///
/// The options are the repository's own analysis_options.yaml files: the nearest at or above the folder and each one
/// on disk below it, gitignored or not, through symlinked folders, outside folders whose name starts with `.`, with
/// what they include (`options_files`). Each is parsed as YAML (`yaml_entries`): block and flow style, every quoting,
/// escapes, scalars over several lines, block scalars, anchors and aliases (of nodes and of keys) are read as YAML
/// reads them. A file the parser cannot read is `Failed` naming the file and the line, and so are, as measured
/// (fixtures/commands.txt, "YAML forms"): a second document, a repeated key and a core tag (`!!int`, `!!bool`,
/// `!!null`; `!!float` not measured) on text that does not read as its type, for which dart drops the whole
/// file (any other `!!` tag, on a scalar or on a collection of another kind, is also `Failed`, unmeasured);
/// a merge key `<<`, which dart reads differently by section; and a key that is not a scalar, which has no text to
/// match while dart still applies an `ignore` beside it. A form tm does not read is never clean. An options
/// file read outside the repository top (the nearest one above it, a symlink's target, a symlinked folder's, an
/// include) is `Failed`, since tm cannot turn it off; a `package:` include, the repository's declared dependency, is
/// read wherever its package lies.
///
/// Suppressions (S-67): a key path under `analyzer: exclude:` and `analyzer: errors: <code>: ignore` in the options,
/// or in a file they include, make the rule `Failed` naming the file and the line, since dart cannot be told to ignore
/// them and exceptions go in the rule file (S-67 (1), `options_suppression`); so does any key `exclude` or any value
/// `ignore`, in any case, wherever it stands. The comments in the code, `// ignore: <code>`,
/// `// ignore_for_file: <code>` and `// ignore: type=lint` (also `// ignore_for_file: type=lint`), are left as they
/// are: dart has no option that turns them off (S-67 (2)), a gap for the baseline's suppression rules.
///
/// Exit codes: dart's exit code is the worst severity found, which its help's `--fatal-infos` and
/// `--[no-]fatal-warnings` (on by default) set; measured with dart 3.14.1 (fixtures/commands.txt): 0 for nothing or
/// infos only (a lint is an info), 2 for warnings, 3 for errors. Exit 0, 2 or 3 with findings is an answer, not an
/// error; any other exit is `Failed` (`parse_dart`).
///
/// Nothing reaches the network: `--suppress-analytics` (in the probe too) turns off dart's analytics for the run
/// without changing the machine's analytics setting, and `dart analyze` never runs `pub get` or fetches a package. It
/// writes nothing in the folder; its analysis cache lives under `~/.dartServer`.
///
/// Settling: a rule whose paths match no file is `no_file_match`'s; a folder with no `.dart` file the rule applies to
/// is `NoFiles` through `engine_listed`, before availability (as clang-tidy); no `pubspec.yaml` at or above the folder,
/// up to the repository top, is `NotAvailable` (`run_dart`). A package whose dependencies were never fetched (no
/// `.dart_tool/package_config.json`) and whose run names a `package:` URI dart cannot find is `NotAvailable`, saying to
/// run `dart pub get` (`missing_packages`), since tm never fetches packages.
///
/// Dart takes no params: any param makes the rule `Failed` before running. Dart rules never share a run:
/// `shares_with` has no arm for it.
pub static DART: Entry = Entry {
    name: "dart",
    engine: Engine::Dart,
    program: "dart",
    needs: &["dart"],
    never: None,
    args: &["--suppress-analytics", "analyze", "--format", "machine"],
    pick: Pick::None,
    params: Params::Args(&[]),
    config: None,
    after: &["."],
    version: Some(VersionProbe { args: &["--suppress-analytics", "--version"], read: version_word, wrap: None }),
    refuse: dart_refusal,
    unknown: |_, _| None,
    read: parse_dart,
    run: run_dart,
};
