//! `remote_dependencies`: the dependencies of a Swift package that SwiftPM would have to fetch.

use crate::printed_start::printed_start;
use crate::types::ParseError;
use procio::Output;
use serde::Deserialize;

/// The part of `swift package dump-package`'s JSON tm reads.
#[derive(Deserialize)]
struct Manifest {
    dependencies: Vec<Dependency>,
}

/// One dependency: an object with one key naming its kind. `fileSystem` (a local path) is not read, since it needs
/// no fetch.
#[derive(Deserialize)]
struct Dependency {
    #[serde(rename = "sourceControl", default)]
    source_control: Vec<Identity>,
    #[serde(default)]
    registry: Vec<Identity>,
}

#[derive(Deserialize)]
struct Identity {
    identity: String,
}

/// The identities of the `sourceControl` and `registry` dependencies in dump-package's output `out`, in the
/// manifest's order; empty when the package depends only on local paths, or on nothing. stdout that is not the
/// manifest is a `ParseError` quoting `printed_start`.
pub fn remote_dependencies(out: &Output) -> Result<Vec<String>, ParseError> {
    let manifest: Manifest = serde_json::from_slice(&out.stdout).map_err(|_| ParseError {
        why: format!("swift package dump-package printed no manifest JSON: {}", printed_start(out)),
    })?;
    Ok(manifest
        .dependencies
        .into_iter()
        .flat_map(|dependency| dependency.source_control.into_iter().chain(dependency.registry))
        .map(|dependency| dependency.identity)
        .collect())
}
