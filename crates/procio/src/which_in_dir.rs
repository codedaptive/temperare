//! `which_in_dir`: where a program would be found inside one given folder. Looks at files only; runs nothing.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::first_in_dir::first_in_dir;
use crate::has_path_separator::has_path_separator;
use crate::names_to_try::names_to_try;
use crate::pathext::pathext;

/// The executable regular file `name` names inside `dir`, found the way `which` finds it in one PATH entry: the file
/// names `names_to_try` gives are tried in order and the first executable regular file wins; `None` when there is
/// none.
///
/// `None`, before anything is read from disk, when:
/// - `dir` is relative: procio never resolves a program against a current directory, as in `which_in`;
/// - `name` is empty or holds a path separator: it is not a file name in `dir` (an empty name would name `dir` itself,
///   with a trailing separator, and under PATHEXT rules `.COM` and the other listed names in `dir`).
///
/// A `dir` that does not exist simply gives `None`, and a directory named `name` is not a program. PATHEXT is this
/// process's own, for the reason `which_in`'s doc gives.
pub fn which_in_dir(dir: &Path, name: &OsStr) -> Option<PathBuf> {
    if !dir.is_absolute() || name.is_empty() || has_path_separator(name) {
        return None;
    }
    first_in_dir(dir, &names_to_try(name, pathext().as_deref()))
}
