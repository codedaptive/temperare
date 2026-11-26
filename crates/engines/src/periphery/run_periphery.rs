//! `run_periphery`: Periphery's flow for one rule. It reads the disk and runs SwiftPM and the engine, so its test is
//! the zone integration test tests/periphery.rs and it has no unit-test sibling.

use super::{answered, build_index_cmd, describe_package_cmd, manifest_walk, package_root, periphery_cmd, ScratchDir};
use crate::check_run::check_run;
use crate::entry::Entry;
use crate::host::Host;
use crate::parse_failed::parse_failed;
use crate::repo_top::repo_top;
use crate::types::{Hit, Stop};
use rules::Rule;
use std::path::Path;

/// The hits of `rule` under `folder` (canonical); a `RunFn`, the flow of `PERIPHERY`. The central `refused` has already
/// settled the rule's refusals. Each step can stop the rule:
/// 1. the package is the nearest `Package.swift` at or above `folder` and no higher than the repository top
///    (`repo_top`) (`package_root`); none is `NotAvailable`, and a `folder` in no repository is `Failed` with
///    ccvcslib's error;
/// 2. a new scratch folder (`ScratchDir`) holds everything the run writes, `<scratch>/build` the build;
/// 3. `manifest_walk` reads the manifest of the package and of each local package it reaches, before anything is
///    built; any of them naming a dependency or a binary target with a url to fetch is `NotAvailable`, since tm never
///    fetches packages, and so is a local package that does not resolve or resolves outside the repository top;
/// 4. `describe_package_cmd`'s description is written to `<scratch>/describe.json`;
/// 5. `build_index_cmd` builds the package; a build that fails is `Failed`, since a package that does not compile
///    gives no answer;
/// 6. the build must have left its index store at `<scratch>/build/out`;
/// 7. an empty `<scratch>/periphery.yml` is written, and `periphery_cmd` is the check run, through `check_run` so it is
///    counted, read by `entry.read` against the package root; output it cannot read is `parse_failed`'s `Failed`.
///
/// A SwiftPM step that exits nonzero is `Failed` with its stderr, or its stdout when stderr is empty. The scratch
/// folder is dropped last, whichever step ends the flow.
pub(crate) fn run_periphery(entry: &Entry, rule: &Rule, folder: &Path, host: &Host) -> Result<Vec<Hit>, Stop> {
    let top = repo_top(folder)?;
    let root = package_root(folder, &top).ok_or_else(|| Stop::NotAvailable {
        why: "no Package.swift at or above the rule file's folder: Periphery builds the project, and tm runs it only on a Swift package".into(),
    })?;
    let scratch = ScratchDir::new().map_err(|err| Stop::Failed {
        code: None,
        stderr: format!("cannot make a scratch folder for Periphery: {err}"),
    })?;
    manifest_walk(&root, &top, &scratch.path, host)?;
    let build = scratch.path.join("build");
    let manifest = scratch.path.join("describe.json");
    written(&manifest, &answered(&describe_package_cmd(&root, &build), host)?.stdout)?;
    answered(&build_index_cmd(&root, &build), host)?;
    let store = build.join("out");
    if !store.is_dir() {
        return Err(Stop::Failed { code: None, stderr: format!("the build left no index store at {}", store.display()) });
    }
    let config = scratch.path.join("periphery.yml");
    written(&config, b"")?;
    let out = check_run(&periphery_cmd(&root, &store, &manifest, &config), host)?;
    (entry.read)(&out, &rule.check, &root, folder).map_err(|err| parse_failed(&out, err))
}

/// Writes `bytes` to the scratch file `path`; an io error is `Failed`, naming the file.
fn written(path: &Path, bytes: &[u8]) -> Result<(), Stop> {
    std::fs::write(path, bytes)
        .map_err(|err| Stop::Failed { code: None, stderr: format!("cannot write {}: {err}", path.display()) })
}
