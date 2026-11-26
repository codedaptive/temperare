//! `which`: where a program would be found on PATH. Looks at files only; runs nothing.

use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;

use crate::first_in_dir::first_in_dir;
use crate::has_path_separator::has_path_separator;
use crate::is_executable::is_executable;
use crate::names_to_try::names_to_try;
use crate::pathext::pathext;

/// The executable file `program` names, looked up on this process's PATH (on Windows with PATHEXT's extensions too);
/// `None` when there is none.
pub fn which(program: impl AsRef<OsStr>) -> Option<PathBuf> {
    which_in(program.as_ref(), env::var_os("PATH").as_deref())
}

/// `which` over a given PATH value, so tests can build their own.
///
/// A program holding a path separator is a path: it is checked as it is and PATH is not consulted. Otherwise each
/// PATH entry is tried in order, and in each the file names `names_to_try` gives, in order; the first executable
/// regular file wins. Only absolute entries are tried: an empty entry (POSIX reads it as the current directory) and a
/// relative one like `bin` or `.` both resolve against a current directory, and procio never resolves a program
/// against one.
///
/// On Windows the names follow PATHEXT: a name already ending in a listed extension (any case) is tried as given,
/// any other with each listed extension appended in PATHEXT order, and a PATHEXT unset or listing none uses
/// `.COM;.EXE;.BAT;.CMD`. Those four are what `run` can start: CreateProcess starts `.exe` and Windows-program `.com`
/// files (a 16-bit `.com` fails as `Spawn`), and Rust's batch handling starts `.bat` and `.cmd`. `.VBS`, `.JS` and the
/// rest need a host program; a PATHEXT that lists them is honoured, and `run` on such a hit fails as `Spawn`. A name
/// holding `:` (like `C:tool`) finds nothing there.
/// PATHEXT is this process's own, not the `Cmd`'s: tm, not the child, chooses which file kinds it starts, and it
/// spawns the full path found, so the child's PATHEXT takes no part. PATH is the `Cmd`'s effective PATH, because it
/// says which folders the `Cmd`'s caller allowed.
pub(crate) fn which_in(program: &OsStr, path_var: Option<&OsStr>) -> Option<PathBuf> {
    if program.is_empty() {
        return None;
    }
    if has_path_separator(program) {
        let path = PathBuf::from(program);
        return is_executable(&path).then_some(path);
    }
    let names = names_to_try(program, pathext().as_deref());
    env::split_paths(path_var?)
        .filter(|dir| dir.is_absolute())
        .find_map(|dir| first_in_dir(&dir, &names))
}
