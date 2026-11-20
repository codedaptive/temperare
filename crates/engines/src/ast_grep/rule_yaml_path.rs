//! `rule_yaml_path`: where a rule's ast-grep YAML is, held inside the rule file's folder.

use std::fs;
use std::path::{Path, PathBuf};

/// The canonical path of the rule YAML `check` under `folder` (canonical). `Err` is the stderr text: a path that cannot
/// be canonicalized cannot be read, and a canonical path outside `folder` (through `..` or a symlink) is refused. An
/// absolute `check` is not refused here because `refused` settles it before any flow; one that got here is still held,
/// since it joins to itself and is refused when outside. The path returned was inside the rule file's folder when it
/// was checked; `read_rule_yaml` opens it with `open_in_folder`, which walks it down from "/" again without following
/// a symlink.
pub(crate) fn rule_yaml_path(folder: &Path, check: &str) -> Result<PathBuf, String> {
    let path = fs::canonicalize(folder.join(check)).map_err(|err| format!("cannot read ast-grep rule {check}: {err}"))?;
    if !path.starts_with(folder) {
        return Err(format!("ast-grep rule {check} is outside the rule file's folder"));
    }
    Ok(path)
}
