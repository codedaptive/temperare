//! Unit tests for install_paths_under.rs: every path under one HOME, as the install contract names them.

use std::path::{Path, PathBuf};

use super::InstallPaths;

#[test]
fn the_paths_under_a_home() {
    assert_eq!(
        InstallPaths::under(Path::new("/home/ann")),
        InstallPaths {
            bin_dir: PathBuf::from("/home/ann/.local/bin"),
            binary: PathBuf::from("/home/ann/.local/bin/cctao"),
            state_dir: PathBuf::from("/home/ann/.local/state/cctao"),
            record: PathBuf::from("/home/ann/.local/state/cctao/installed"),
        }
    );
}
