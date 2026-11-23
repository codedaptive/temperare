//! `on_tm_path`: whether a program is on the PATH `temperare` gets in acceptance (`clean_env`), or, given as an
//! absolute path, whether that file is there. It looks at files only, as `temperare`'s own lookup does (procio's
//! `which`: an executable regular file), and runs nothing.

use crate::tm_path_folder::tm_path_folder;

/// True when a folder on the PATH `clean_env(None)` gives `temperare` holds an executable regular file named `program`;
/// for an absolute `program`, when that file is an executable regular file (`tm_path_folder`).
pub fn on_tm_path(program: &str) -> bool {
    tm_path_folder(program).is_some()
}
