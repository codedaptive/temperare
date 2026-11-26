//! `discover_rule_file`: which rule file tm reads.

use crate::UsageError;
use ccvcslib::VcsError;
use std::path::{Path, PathBuf};

/// The `--rules` path joined to `cwd` when given (whether it exists is `RuleFile::read`'s to say), else
/// `temperare.toml` at the top of the git repository holding `cwd`. `toplevel` and `exists` are the git and disk
/// seams, so the decision itself does no I/O; neither is called when the flag is given.
pub fn discover_rule_file(
    flag: Option<&Path>,
    cwd: &Path,
    toplevel: &dyn Fn() -> Result<PathBuf, VcsError>,
    exists: &dyn Fn(&Path) -> bool,
) -> Result<PathBuf, UsageError> {
    if let Some(path) = flag {
        return Ok(cwd.join(path));
    }
    let top = toplevel().map_err(|e| match e {
        VcsError::NotAWorkingCopy => usage_error(
            format!("not in a git repository: {}", cwd.display()),
            "Run tm inside a git repository, or pass --rules <path>.",
        ),
        e => usage_error(
            format!("git could not find the repository top: {e}"),
            "Check that git runs here, or pass --rules <path>.",
        ),
    })?;
    let path = top.join("temperare.toml");
    if !exists(&path) {
        return Err(usage_error(
            format!("no rule file at {}", path.display()),
            "Write temperare.toml at the top of the repository, or pass --rules <path>.",
        ));
    }
    Ok(path)
}

fn usage_error(what: String, fix: &str) -> UsageError {
    UsageError { what, fix: fix.to_string() }
}
