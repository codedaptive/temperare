//! `options_files`: the analysis options that govern what dart reports under a rule's folder. It reads the disk, so
//! its test is the zone integration test tests/dart.rs and it has no unit-test sibling.

use super::{include_targets, package_path, pubspec_root};
use crate::cannot_turn_off::CANNOT_TURN_OFF;
use crate::types::Stop;
use std::collections::HashSet;
use std::fs::{DirEntry, Metadata};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

/// The options file dart reads, by name.
const OPTIONS: &str = "analysis_options.yaml";

/// One chain per analysis_options.yaml that governs a file under `folder` (canonical), each file of a chain its path
/// and text: the files it includes first, depth first in the order written and each once (an include cycle ends), the
/// governing file last. Dart 3.14.1 reads the nearest analysis_options.yaml at or above each file, with no bound at the
/// repository or package, and merges another only through `include:`; so the governing files are the nearest one at
/// or above `folder`, if any, then each one on disk below `folder` (gitignored too, as dart reads it), in path order.
/// The walk follows symlinked folders, as dart does, once per canonical folder, and skips folders whose name starts
/// with `.`, which dart does not analyze. An analysis_options.yaml that is a symlink is read through it; a broken one
/// is passed over, as dart passes it over for the next one up (fixtures/commands.txt). Each step fails closed:
/// - a file the chain reads whose canonical path lies outside `top`, the repository top (canonical), is `Failed`
///   naming it as found and as read, since cctao cannot turn it off (S-67 (1)); but a `package:` include, and what it
///   includes, is the repository's declared dependency and is read wherever the package config puts it;
/// - an include is read beside the file that includes it, or as a `package:` URI through the
///   `.dart_tool/package_config.json` (`package_path`) of the nearest pubspec.yaml at or above the governing file and
///   no higher than `top` (`pubspec_root`), else of `package_root`, the rule's package; no such file, or a URI it does
///   not resolve, is `NotAvailable` saying to run `dart pub get`, since cctao never fetches packages;
/// - a file it cannot read, a folder it cannot list, or a path whose metadata fails for a reason other than absence is
///   `NotAvailable` naming it; a file whose YAML cctao cannot read (`include_targets`) is `Failed` naming the file and
///   the line.
pub(crate) fn options_files(
    folder: &Path,
    top: &Path,
    package_root: &Path,
) -> Result<Vec<Vec<(PathBuf, String)>>, Stop> {
    let mut governing: Vec<PathBuf> =
        folder.ancestors().map(options_in).find_map(Result::transpose).transpose()?.into_iter().collect();
    nested(folder, &mut HashSet::from([folder.to_path_buf()]), &mut governing)?;
    governing
        .iter()
        .map(|file| {
            let root = file.parent().and_then(|dir| pubspec_root(dir, top));
            let mut chain = Vec::new();
            followed(file, top, root.as_deref().unwrap_or(package_root), false, &mut HashSet::new(), &mut chain)?;
            Ok(chain)
        })
        .collect()
}

/// `dir`'s analysis_options.yaml when it is a file, through a symlink; None when absent, a folder or a broken link.
fn options_in(dir: &Path) -> Result<Option<PathBuf>, Stop> {
    let file = dir.join(OPTIONS);
    Ok(present(&file)?.filter(Metadata::is_file).map(|_| file))
}

/// `path`'s metadata through a symlink; None when absent (a broken link too); else `NotAvailable` naming it.
fn present(path: &Path) -> Result<Option<Metadata>, Stop> {
    match std::fs::metadata(path) {
        Ok(meta) => Ok(Some(meta)),
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
        Err(err) => Err(Stop::NotAvailable {
            why: format!("cannot read {} while looking for analysis options files: {err}", path.display()),
        }),
    }
}

/// Appends to `found`, in path order, the analysis_options.yaml of each folder below `dir`, walking folders, symlinked
/// ones included, whose name does not start with `.` and whose canonical path is not in `visited`.
fn nested(dir: &Path, visited: &mut HashSet<PathBuf>, found: &mut Vec<PathBuf>) -> Result<(), Stop> {
    let unlisted = |err: std::io::Error| Stop::NotAvailable {
        why: format!("cannot list the folder {} for its analysis options files: {err}", dir.display()),
    };
    let mut entries = std::fs::read_dir(dir).map_err(unlisted)?.collect::<Result<Vec<_>, _>>().map_err(unlisted)?;
    entries.sort_by_key(DirEntry::path);
    for entry in entries {
        let path = entry.path();
        if entry.file_name().as_encoded_bytes().starts_with(b".") || !present(&path)?.is_some_and(|m| m.is_dir()) {
            continue;
        }
        if visited.insert(std::fs::canonicalize(&path).map_err(unlisted)?) {
            found.extend(options_in(&path)?);
            nested(&path, visited, found)?;
        }
    }
    Ok(())
}

/// Appends to `chain` what `file` includes, then `file` itself, skipping a file already in `seen` (canonical paths).
/// `declared` says `file` is reached through a `package:` include, so it may lie outside `top`.
fn followed(
    file: &Path,
    top: &Path,
    root: &Path,
    declared: bool,
    seen: &mut HashSet<PathBuf>,
    chain: &mut Vec<(PathBuf, String)>,
) -> Result<(), Stop> {
    let unread = |err: std::io::Error| Stop::NotAvailable {
        why: format!("cannot read the analysis options file {}: {err}", file.display()),
    };
    let canonical = std::fs::canonicalize(file).map_err(unread)?;
    if !declared && !canonical.starts_with(top) {
        let read_as = if canonical == file { String::new() } else { format!(" (read as {})", canonical.display()) };
        return Err(Stop::Failed {
            code: None,
            stderr: format!(
                "{}{read_as}, outside the repository top, governs what dart reports here and {CANNOT_TURN_OFF}",
                file.display()
            ),
        });
    }
    if !seen.insert(canonical) {
        return Ok(());
    }
    let text = std::fs::read_to_string(file).map_err(unread)?;
    let targets = include_targets(&text)
        .map_err(|why| Stop::Failed { code: None, stderr: format!("{}: {why}", file.display()) })?;
    for target in targets {
        let declared = declared || target.starts_with("package:");
        followed(&resolved(&target, file, root)?, top, root, declared, seen, chain)?;
    }
    chain.push((file.to_path_buf(), text));
    Ok(())
}

/// The file `target`, included by `from`, names: beside `from` (`with_file_name` joins a relative target to `from`'s
/// folder and keeps an absolute one), or for a `package:` URI through the package config of the package `root`.
fn resolved(target: &str, from: &Path, root: &Path) -> Result<PathBuf, Stop> {
    if !target.starts_with("package:") {
        return Ok(from.with_file_name(target));
    }
    let config = root.join(".dart_tool").join("package_config.json");
    let not_resolved = |what: String| Stop::NotAvailable {
        why: format!(
            "dart cannot resolve the include `{target}` in {}: {what}; run `dart pub get` once in the package, \
             since cctao never fetches packages",
            from.display()
        ),
    };
    if !config.is_file() {
        let what = "the package's dependencies are not fetched (no .dart_tool/package_config.json)";
        return Err(not_resolved(what.into()));
    }
    let json = std::fs::read_to_string(&config)
        .map_err(|err| not_resolved(format!("cannot read {}: {err}", config.display())))?;
    package_path(target, &json, &root.join(".dart_tool"), cfg!(windows))
        .ok_or_else(|| not_resolved(format!("{} does not resolve it", config.display())))
}
