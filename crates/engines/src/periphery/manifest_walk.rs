//! `manifest_walk`: every local package a Swift package reaches, read before anything is built. Its unit-test sibling
//! manifest_walk_tests.rs runs it on a fake host's dump-package output; the zone integration test tests/periphery.rs
//! runs it on SwiftPM. Its own disk use is canonicalizing the local paths it follows.

use super::{answered, dump_package_cmd, local_dependencies, package_subject, remote_dependencies};
use crate::host::Host;
use crate::parse_failed::parse_failed;
use crate::types::Stop;
use std::collections::{HashSet, VecDeque};
use std::path::{Path, PathBuf};

/// Reads the manifest of the package at `root` (canonical) and of each local package it reaches, breadth-first, each
/// once: a visited set of canonical paths, starting with `root`, ends a cycle of local packages (SwiftPM builds one:
/// fixtures/commands.txt). Each package's `dump_package_cmd` gets a scratch path of its own under `scratch`: the
/// root's `<scratch>/build`, the one its build uses, then `<scratch>/dep-<n>` for the n-th package after it. Each
/// package can stop the rule, `<subject>` being `package_subject`'s: "the package" for the root and "the local package
/// <its path relative to `top`>" for any other, "the local package ." for a package at `top` itself:
/// - a dependency or url binary target to fetch (`remote_dependencies`) is `NotAvailable`, since tm never fetches
///   packages;
/// - a local package (`local_dependencies`) whose path does not resolve, or resolves outside `top` (the repository
///   top, canonical), is `NotAvailable`: tm cannot vouch for code it cannot read or that is not the repository's.
///
/// A dump-package that exits nonzero is `answered`'s `Failed`; output that is not a manifest is `parse_failed`'s.
pub(crate) fn manifest_walk(root: &Path, top: &Path, scratch: &Path, host: &Host) -> Result<(), Stop> {
    let mut visited = HashSet::from([root.to_path_buf()]);
    let mut queue = VecDeque::from([root.to_path_buf()]);
    let mut n = 0;
    while let Some(package) = queue.pop_front() {
        let build = if package == root {
            scratch.join("build")
        } else {
            n += 1;
            scratch.join(format!("dep-{n}"))
        };
        let subject = package_subject(&package, root, top);
        let dump = answered(&dump_package_cmd(&package, &build), host)?;
        let remote = remote_dependencies(&dump).map_err(|err| parse_failed(&dump, err))?;
        if !remote.is_empty() {
            return Err(Stop::NotAvailable {
                why: format!(
                    "{subject} depends on {}, which tm would have to fetch: tm never fetches packages, so Periphery cannot build it here",
                    remote.join(", ")
                ),
            });
        }
        for path in local_dependencies(&dump).map_err(|err| parse_failed(&dump, err))? {
            let local = vouched(&path, &subject, top)?;
            if visited.insert(local.clone()) {
                queue.push_back(local);
            }
        }
    }
    Ok(())
}

/// The canonical path of the local package `path` that `subject` depends on; one that does not resolve, or resolves
/// outside `top`, is `NotAvailable`, naming it.
fn vouched(path: &Path, subject: &str, top: &Path) -> Result<PathBuf, Stop> {
    let local = std::fs::canonicalize(path).map_err(|_| Stop::NotAvailable {
        why: format!(
            "{subject} depends on the local package {}, which does not resolve: tm cannot vouch for it, so Periphery cannot build it here",
            path.display()
        ),
    })?;
    if !local.starts_with(top) {
        return Err(Stop::NotAvailable {
            why: format!(
                "{subject} depends on the local package {}, outside the repository: tm cannot vouch for it, so Periphery cannot build it here",
                local.display()
            ),
        });
    }
    Ok(local)
}
