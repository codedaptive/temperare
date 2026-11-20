//! `tm_owner`: another program that answers to `tm` on PATH.
//!
//! No sibling unit test: it reads the disk, which a unit test may not, so the zone integration target
//! `tests/install` (placement.rs: the clash, `--short-name`, our own link on PATH, an empty PATH, an unset PATH, empty
//! and relative PATH entries) proves it on a temporary HOME.

use std::ffi::OsStr;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

/// The first `tm` in the folders of `path_var`, read as POSIX does (an empty entry is `cwd`, a relative entry is
/// `cwd.join(entry)`, so a PATH set to "" is one entry, `cwd`), that is an executable file, following links, is not
/// `link` itself, and is not the same file as any of `ours` (both canonicalized; an entry of `ours` that does not
/// exist is skipped). A `path_var` of `None` (PATH unset) is no search. `None` when there is no such `tm`: linking
/// `tm` takes it from nobody.
pub(crate) fn tm_owner(path_var: Option<&OsStr>, cwd: &Path, link: &Path, ours: &[PathBuf]) -> Option<PathBuf> {
    let path_var = path_var?;
    let ours: Vec<PathBuf> = ours.iter().filter_map(|path| fs::canonicalize(path).ok()).collect();
    std::env::split_paths(path_var)
        .map(|dir| cwd.join(dir).join("tm"))
        .find(|candidate| candidate != link && is_executable_file(candidate) && !is_ours(candidate, &ours))
}

/// A regular file, following links, with any execute bit set.
#[cfg(unix)]
fn is_executable_file(path: &Path) -> bool {
    fs::metadata(path).is_ok_and(|meta| meta.is_file() && meta.permissions().mode() & 0o111 != 0)
}

/// A regular file, following links; off unix there is no execute bit to read.
#[cfg(not(unix))]
fn is_executable_file(path: &Path) -> bool {
    fs::metadata(path).is_ok_and(|meta| meta.is_file())
}

/// Whether `path` resolves to one of `ours` (already canonical).
fn is_ours(path: &Path, ours: &[PathBuf]) -> bool {
    fs::canonicalize(path).is_ok_and(|real| ours.contains(&real))
}
