//! `local_dependencies`: the local packages a Swift package's manifest depends on by path.

use crate::printed_start::printed_start;
use crate::types::ParseError;
use procio::Output;
use serde::Deserialize;
use std::path::PathBuf;

/// The part of `swift package dump-package`'s JSON tm reads here.
#[derive(Deserialize)]
struct Manifest {
    dependencies: Vec<Dependency>,
}

/// One dependency: an object with one key naming its kind; only `fileSystem` (a local path) is read.
#[derive(Deserialize)]
struct Dependency {
    #[serde(rename = "fileSystem", default)]
    file_system: Vec<Local>,
}

#[derive(Deserialize)]
struct Local {
    path: PathBuf,
}

/// The paths of the `fileSystem` dependencies in dump-package's output `out`, in the manifest's order, as SwiftPM
/// printed them (absolute, not canonicalized). Empty when the package names no local path. stdout that is not the
/// manifest is a `ParseError` quoting `printed_start`.
pub fn local_dependencies(out: &Output) -> Result<Vec<PathBuf>, ParseError> {
    let manifest: Manifest = serde_json::from_slice(&out.stdout).map_err(|_| ParseError {
        why: format!("swift package dump-package printed no manifest JSON: {}", printed_start(out)),
    })?;
    Ok(manifest.dependencies.into_iter().flat_map(|dependency| dependency.file_system).map(|local| local.path).collect())
}
