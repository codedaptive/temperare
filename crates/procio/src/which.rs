//! `which`: where a program would be found on PATH. Looks at files only; runs nothing.

use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// The executable file `program` names, looked up on this process's PATH; `None` when there is none.
pub fn which(program: impl AsRef<OsStr>) -> Option<PathBuf> {
    which_in(program.as_ref(), env::var_os("PATH").as_deref())
}

/// `which` over a given PATH value, so tests can build their own.
///
/// A program holding a path separator is a path: it is checked as it is and PATH is not consulted. Otherwise each
/// PATH entry is tried in order and the first executable regular file wins. Empty entries are skipped: POSIX reads
/// them as the current directory, and procio never resolves a program against its own current directory.
pub(crate) fn which_in(program: &OsStr, path_var: Option<&OsStr>) -> Option<PathBuf> {
    if program.is_empty() {
        return None;
    }
    if program.to_string_lossy().chars().any(std::path::is_separator) {
        let path = PathBuf::from(program);
        return is_executable(&path).then_some(path);
    }
    env::split_paths(path_var?)
        .filter(|dir| !dir.as_os_str().is_empty())
        .map(|dir| dir.join(program))
        .find(|candidate| is_executable(candidate))
}

/// A regular file with an execute bit set for anyone.
#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    path.metadata().is_ok_and(|meta| meta.is_file() && meta.permissions().mode() & 0o111 != 0)
}

/// A regular file: outside unix there is no execute bit to read.
#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}
