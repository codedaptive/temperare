//! `InstallPaths`: where install puts the binary and its record (S-32).

use std::path::PathBuf;

/// The places under HOME install uses: `bin_dir` (`.local/bin`), `binary` (`cctao` there), `state_dir`
/// (`.local/state/cctao`) and `record` (`installed` there, the list of what install placed).
/// `InstallPaths::under` (install_paths_under.rs) makes them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallPaths {
    pub bin_dir: PathBuf,
    pub binary: PathBuf,
    pub state_dir: PathBuf,
    pub record: PathBuf,
}
