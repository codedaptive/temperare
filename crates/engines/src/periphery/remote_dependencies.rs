//! `remote_dependencies`: what a Swift package's manifest names that SwiftPM would have to fetch.

use crate::printed_start::printed_start;
use crate::types::ParseError;
use procio::Output;
use serde::Deserialize;

/// The part of `swift package dump-package`'s JSON tm reads.
#[derive(Deserialize)]
struct Manifest {
    dependencies: Vec<Dependency>,
    targets: Vec<Target>,
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

/// One target: a binary target has `"type": "binary"` and either a `url` (fetched) or a `path` (local, no url).
#[derive(Deserialize)]
struct Target {
    name: String,
    #[serde(rename = "type")]
    kind: String,
    url: Option<String>,
}

/// What dump-package's output `out` names to fetch, in the manifest's order: the identities of its `sourceControl`
/// and `registry` dependencies, then `binary target <name> (<url>)` for each binary target with a url. Empty when
/// the package names only local paths, or nothing. It reads one manifest; the walk (`manifest_walk`) reads each.
/// stdout that is not the manifest is a `ParseError` quoting `printed_start`.
pub fn remote_dependencies(out: &Output) -> Result<Vec<String>, ParseError> {
    let manifest: Manifest = serde_json::from_slice(&out.stdout).map_err(|_| ParseError {
        why: format!("swift package dump-package printed no manifest JSON: {}", printed_start(out)),
    })?;
    let dependencies = manifest
        .dependencies
        .into_iter()
        .flat_map(|dependency| dependency.source_control.into_iter().chain(dependency.registry))
        .map(|dependency| dependency.identity);
    let binaries = manifest.targets.into_iter().filter(|target| target.kind == "binary").filter_map(|target| {
        target.url.map(|url| format!("binary target {} ({url})", target.name))
    });
    Ok(dependencies.chain(binaries).collect())
}
