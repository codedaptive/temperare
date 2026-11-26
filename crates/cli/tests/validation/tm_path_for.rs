//! `tm_path_for`: the narrow PATH a Done 9 case gives `temperare`: only the folders that hold the programs it needs,
//! then /usr/bin:/bin (git). A full PATH has hung tm's version probe, so no other folder is on it.

use crate::tm_path_folder::tm_path_folder;

/// The folders `tm_path_folder` gives for each of `programs`, in order, each folder once, then `/usr/bin:/bin`. A
/// program with no folder adds nothing, so with none found the PATH is `/usr/bin:/bin`.
pub fn tm_path_for(programs: &[&str]) -> String {
    let mut dirs: Vec<String> = Vec::new();
    for dir in programs.iter().filter_map(|program| tm_path_folder(program)) {
        if !dirs.contains(&dir) {
            dirs.push(dir);
        }
    }
    dirs.push("/usr/bin:/bin".to_string());
    dirs.join(":")
}
