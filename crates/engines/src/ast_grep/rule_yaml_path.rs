//! `rule_yaml_path`: where a rule's ast-grep YAML is, held inside the rule file's folder.

use std::fs;
use std::path::{Path, PathBuf};

/// The canonical path of the rule YAML `check` under `folder` (canonical). `Err` is the stderr text: an absolute
/// `check` is refused before the filesystem is touched, a path that cannot be canonicalized cannot be read, and a
/// canonical path outside `folder` (through `..` or a symlink) is refused. The path returned was inside the rule
/// file's folder when it was checked; `read_rule_yaml` opens it with `open_in_folder`, which walks it down from
/// "/" again without following a symlink.
pub(crate) fn rule_yaml_path(folder: &Path, check: &str) -> Result<PathBuf, String> {
    if Path::new(check).is_absolute() {
        return Err(format!(
            "ast-grep rule {check} is an absolute path: a check names a file inside the rule file's folder"
        ));
    }
    let path = fs::canonicalize(folder.join(check)).map_err(|err| format!("cannot read ast-grep rule {check}: {err}"))?;
    if !path.starts_with(folder) {
        return Err(format!("ast-grep rule {check} is outside the rule file's folder"));
    }
    Ok(path)
}
