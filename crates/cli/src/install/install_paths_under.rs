//! `InstallPaths::under`: the install paths for one HOME.

use std::path::Path;

use super::InstallPaths;

impl InstallPaths {
    /// The paths under `home`: the XDG-style user bin and state folders, no root needed. Install writes nothing to
    /// `.config/cctao`, so it has no path here.
    pub fn under(home: &Path) -> InstallPaths {
        let bin_dir = home.join(".local").join("bin");
        let state_dir = home.join(".local").join("state").join("cctao");
        InstallPaths {
            binary: bin_dir.join("cctao"),
            record: state_dir.join("installed"),
            bin_dir,
            state_dir,
        }
    }
}
