//! `PERIPHERY`: the Periphery engine as an entry.

use super::{parse_periphery, periphery_refusal, run_periphery};
use crate::entry::{version_word, Entry, Params, Pick, VersionProbe};
use rules::Engine;

/// Periphery run by its own flow, `run_periphery`, on the Swift package at or above the rule file's folder inside its
/// repository (none is not available). Periphery finds unused code from a build's index store, so a rule takes these
/// runs: `swift package dump-package` (`dump_package_cmd`) once for each package in the walk (`manifest_walk`), in
/// that package's folder; then, in the package root, `swift package describe --type json` (`describe_package_cmd`),
/// `swift build --build-system swiftbuild` (`build_index_cmd`), and
/// `periphery scan --skip-build --format json --disable-update-check --quiet` with the index store, the description
/// and a config (`periphery_cmd`), read by `parse_periphery`.
///
/// Where everything goes: a scratch folder `tm-periphery-<pid>-<n>` under the system's temporary folder holds the
/// build, its index store, the local packages' dump-package scratch paths, the description and the config, and is
/// removed after the run whatever its end (`ScratchDir`). Nothing is built inside the repository being checked, and nothing goes to the user's DerivedData.
/// The build is tm's step, not Periphery's, because Periphery's own build looks for the index store under the
/// package's `.build`.
///
/// Offline: `--disable-update-check` keeps Periphery from asking for a newer release. The manifest of the package and
/// of every local package it reaches is read (`manifest_walk`), and the package is not available, before anything is
/// built, when any of them names a source-control or registry dependency or a binary target with a url
/// (`remote_dependencies`), since tm never fetches packages, or depends on a local package outside the repository or
/// not resolving, since tm cannot vouch for it. Binary targets are refused because, run with the network denied,
/// `swift build --disable-automatic-resolution` still tries to download a remote binary artifact. The build's
/// `--disable-automatic-resolution` fails rather than resolve a dependency. No environment is set.
///
/// Exceptions live in the rule file only: `--config` names an empty file tm writes, so the folder's `.periphery.yml`
/// (whose retain, exclude and report-exclude options are exceptions outside the rule file) is never read. Periphery
/// still honours `// periphery:ignore` comments in the source; no option turns them off.
///
/// A check is a result hint of Periphery 3.8.0, written as Periphery writes it (`unused`, `assignOnlyProperty`,
/// `redundantProtocol`, `redundantConformance`, `redundantPublicAccessibility`, `superfluousIgnoreCommand`) or `all`;
/// anything else is refused before anything runs (`periphery_refusal`). The check is never passed to Periphery: it
/// reports every result and the reader keeps the rule's. `periphery` missing is the engine not on PATH; `swift`
/// missing is a helper not on PATH.
pub static PERIPHERY: Entry = Entry {
    name: "periphery",
    engine: Engine::Periphery,
    program: "periphery",
    needs: &["periphery"],
    never: None,
    args: &["scan", "--skip-build", "--format", "json", "--disable-update-check", "--quiet"],
    pick: Pick::None,
    params: Params::Args(&[]),
    config: None,
    after: &[],
    version: Some(VersionProbe { args: &["version"], read: version_word, wrap: None }),
    refuse: periphery_refusal,
    unknown: |_, _| None,
    read: parse_periphery,
    run: run_periphery,
};
