//! `run_periphery`: Periphery's flow for one rule. It reads the disk and runs SwiftPM and the engine, so its test is
//! the zone integration test tests/periphery.rs and it has no unit-test sibling.

use super::{
    build_index_cmd, describe_package_cmd, dump_package_cmd, package_root, periphery_cmd, remote_dependencies,
    ScratchDir,
};
use crate::check_run::check_run;
use crate::entry::Entry;
use crate::host::Host;
use crate::parse_failed::parse_failed;
use crate::ran::ran;
use crate::types::{Hit, Stop};
use procio::{Cmd, Output};
use rules::Rule;
use std::path::Path;

/// The hits of `rule` under `folder` (canonical); a `RunFn`, the flow of `PERIPHERY`. The central `refused` has already
/// settled the rule's refusals. Each step can stop the rule:
/// 1. the package is the nearest `Package.swift` at or above `folder` (`package_root`); none is `NotAvailable`;
/// 2. a new scratch folder (`ScratchDir`) holds everything the run writes, `<scratch>/build` the build;
/// 3. `dump_package_cmd`'s manifest naming a dependency to fetch (`remote_dependencies`) is `NotAvailable`: tm never
///    fetches packages;
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
    let root = package_root(folder).ok_or_else(|| Stop::NotAvailable {
        why: "no Package.swift at or above the rule file's folder: Periphery builds the project, and tm runs it only on a Swift package".into(),
    })?;
    let scratch = ScratchDir::new().map_err(|err| Stop::Failed {
        code: None,
        stderr: format!("cannot make a scratch folder for Periphery: {err}"),
    })?;
    let build = scratch.path.join("build");
    let dump = answered(&dump_package_cmd(&root, &build), host)?;
    let remote = remote_dependencies(&dump).map_err(|err| parse_failed(&dump, err))?;
    if !remote.is_empty() {
        return Err(Stop::NotAvailable {
            why: format!(
                "the package depends on {}, which tm would have to fetch: tm never fetches packages, so Periphery cannot build it here",
                remote.join(", ")
            ),
        });
    }
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

/// The output of a SwiftPM step that exited 0; any other exit is `Failed` with its code and its stderr, or its stdout
/// when stderr is empty (SwiftPM prints some build errors there).
fn answered(cmd: &Cmd, host: &Host) -> Result<Output, Stop> {
    let out = ran(cmd, host)?;
    if out.code == Some(0) {
        return Ok(out);
    }
    let printed = if out.stderr.is_empty() { &out.stdout } else { &out.stderr };
    Err(Stop::Failed { code: out.code, stderr: String::from_utf8_lossy(printed).into_owned() })
}

/// Writes `bytes` to the scratch file `path`; an io error is `Failed`, naming the file.
fn written(path: &Path, bytes: &[u8]) -> Result<(), Stop> {
    std::fs::write(path, bytes)
        .map_err(|err| Stop::Failed { code: None, stderr: format!("cannot write {}: {err}", path.display()) })
}
