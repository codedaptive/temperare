//! `rule_yaml_path`: where a rule's ast-grep YAML is, held inside the rule file's folder.

use std::fs;
use std::path::{Path, PathBuf};

/// The canonical path of the rule YAML `check` under `folder` (canonical). `Err` is the stderr text: a path that cannot
/// be canonicalized (a missing file among them) cannot be read, and a canonical path outside `folder` is refused.
/// `refused` settles an absolute check and a check whose `..` climbs above the folder before any flow, from the text
/// alone; the canonical check here is what catches a symlink that leads outside, and a missing file, since both need
/// the disk. A check that got here unsettled is still held: it is refused when its canonical path is outside. The path
/// returned was inside the rule file's folder when it
/// was checked; `read_rule_yaml` opens it with `open_in_folder`, which walks it down from "/" again without following
/// a symlink.
pub(crate) fn rule_yaml_path(folder: &Path, check: &str) -> Result<PathBuf, String> {
    let path = fs::canonicalize(folder.join(check)).map_err(|err| format!("cannot read ast-grep rule {check}: {err}"))?;
    if !path.starts_with(folder) {
        return Err(format!("ast-grep rule {check} is outside the rule file's folder"));
    }
    Ok(path)
}
