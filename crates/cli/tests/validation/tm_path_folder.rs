//! `tm_path_folder`: the folder that gives `temperare` a program on the PATH it gets in acceptance (`clean_env`). It
//! looks at files only, as `temperare`'s own lookup does (procio's `which`: an executable regular file), and runs
//! nothing.

use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::env::clean_env;

/// The first folder on the PATH `clean_env(None)` gives `temperare` that holds an executable regular file named
/// `program`; None when no folder does. A `program` given as an absolute path is that file, wherever the PATH points:
/// its parent folder when it is an executable regular file, else None.
pub fn tm_path_folder(program: &str) -> Option<String> {
    let file = Path::new(program);
    if file.is_absolute() {
        return file.parent().filter(|_| executable(file)).map(|dir| dir.display().to_string());
    }
    let path = clean_env(None).into_iter().find(|(k, _)| k == "PATH").map(|(_, v)| v).unwrap_or_default();
    path.split(':')
        .filter(|dir| !dir.is_empty())
        .find(|dir| executable(&Path::new(dir).join(program)))
        .map(str::to_string)
}

/// A regular file with an execute bit set for anyone.
fn executable(file: &Path) -> bool {
    file.metadata().is_ok_and(|meta| meta.is_file() && meta.permissions().mode() & 0o111 != 0)
}
