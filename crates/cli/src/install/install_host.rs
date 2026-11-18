//! `InstallHost`: what install and uninstall read from the machine, given as input so the logic reads no environment.

use std::ffi::OsString;
use std::path::PathBuf;

/// The machine as install sees it: `home` (HOME), `path_var` (PATH: `None` when it is unset, else its value as
/// given, an empty one included), `exe` (the running binary), `cwd` (the project for `--location local`), the
/// package-manager `prefixes`, and whether a person is at a terminal to answer the harness picker.
/// `InstallHost::real` (install_host_real.rs) reads the real values; tests build their own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallHost {
    pub home: PathBuf,
    pub path_var: Option<OsString>,
    pub exe: PathBuf,
    pub cwd: PathBuf,
    pub prefixes: Vec<PathBuf>,
    pub interactive: bool,
}
