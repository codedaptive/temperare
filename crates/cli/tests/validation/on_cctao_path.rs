//! `on_cctao_path`: whether a program is on the PATH `cctao` gets in acceptance (`clean_env`), or, given as an
//! absolute path, whether that file is there. It looks at files only, as `cctao`'s own lookup does (procio's
//! `which`: an executable regular file), and runs nothing.

use crate::cctao_path_folder::cctao_path_folder;

/// True when a folder on the PATH `clean_env(None)` gives `cctao` holds an executable regular file named `program`;
/// for an absolute `program`, when that file is an executable regular file (`cctao_path_folder`).
pub fn on_cctao_path(program: &str) -> bool {
    cctao_path_folder(program).is_some()
}
