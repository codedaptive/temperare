//! `on_tm_path`: whether a program is on the PATH `tm` gets in acceptance (`clean_env`). It looks at files only, as
//! `tm`'s own lookup does (procio's `which`: an executable regular file), and runs nothing.

use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::env::clean_env;

/// True when a folder on the PATH `clean_env(None)` gives `tm` holds an executable regular file named `program`.
pub fn on_tm_path(program: &str) -> bool {
    let path = clean_env(None).into_iter().find(|(k, _)| k == "PATH").map(|(_, v)| v).unwrap_or_default();
    path.split(':').filter(|dir| !dir.is_empty()).any(|dir| executable(&Path::new(dir).join(program)))
}

/// A regular file with an execute bit set for anyone.
fn executable(file: &Path) -> bool {
    file.metadata().is_ok_and(|meta| meta.is_file() && meta.permissions().mode() & 0o111 != 0)
}
