//! `InstallPaths`: where install puts the binary, the `tm` link and its record (S-32).

use std::path::PathBuf;

/// The places under HOME install uses: `bin_dir` (`.local/bin`), `binary` (`temperare` there), `link` (`tm` there),
/// `state_dir` (`.local/state/temperare`) and `record` (`installed` there, the list of what install placed).
/// `InstallPaths::under` (install_paths_under.rs) makes them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallPaths {
    pub bin_dir: PathBuf,
    pub binary: PathBuf,
    pub link: PathBuf,
    pub state_dir: PathBuf,
    pub record: PathBuf,
}
