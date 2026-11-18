//! `InstallHost::real`: the install host from the real environment, terminal and running binary.

use std::ffi::OsString;
use std::io::{self, IsTerminal};
use std::path::{Path, PathBuf};

use super::{InstallHost, PACKAGE_PREFIXES};
use crate::UsageError;

impl InstallHost {
    /// HOME, PATH, the running binary (links resolved, so `tm` reads as the file it points at), `cwd`, the
    /// package-manager prefixes, and interactive when stdin and stderr are both terminals: the harness picker reads
    /// its answer on stdin and asks on stderr. A `UsageError` when HOME is unset, empty or not absolute, or the
    /// running binary cannot be found, its `what` and `fix` by cause; `host_from` decides.
    pub fn real(cwd: &Path) -> Result<InstallHost, UsageError> {
        host_from(
            std::env::var_os("HOME"),
            std::env::var_os("PATH"),
            std::env::current_exe().and_then(std::fs::canonicalize),
            cwd,
            io::stdin().is_terminal() && io::stderr().is_terminal(),
        )
    }
}

/// The host from the values `real` read. HOME unset or empty, checked first (install has nowhere to put anything):
/// what `HOME is not set`, fix to set HOME. HOME not absolute, checked next (the install record holds paths under
/// it, and its reader keeps absolute ones only): what `HOME is not an absolute path: <home>`, fix to set HOME to an
/// absolute path. The binary's error `e`: what `cannot find the running binary: <e>`, fix to run temperare from its
/// file. PATH is kept as read: `None` when unset, an empty value as `Some` of it.
pub(crate) fn host_from(
    home: Option<OsString>,
    path_var: Option<OsString>,
    exe: io::Result<PathBuf>,
    cwd: &Path,
    interactive: bool,
) -> Result<InstallHost, UsageError> {
    let home = home.filter(|home| !home.is_empty()).ok_or_else(|| UsageError {
        what: "HOME is not set".to_string(),
        fix: "Set HOME to your home folder, then run the command again.".to_string(),
    })?;
    let home = PathBuf::from(home);
    if !home.is_absolute() {
        return Err(UsageError {
            what: format!("HOME is not an absolute path: {}", home.display()),
            fix: "Set HOME to the absolute path of your home folder, then run the command again.".to_string(),
        });
    }
    let exe = exe.map_err(|e| UsageError {
        what: format!("cannot find the running binary: {e}"),
        fix: "Run temperare from its file (not through a deleted or moved path), then try again.".to_string(),
    })?;
    Ok(InstallHost {
        home,
        path_var,
        exe,
        cwd: cwd.to_path_buf(),
        prefixes: PACKAGE_PREFIXES.iter().map(PathBuf::from).collect(),
        interactive,
    })
}
