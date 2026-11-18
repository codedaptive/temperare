//! `rule_yaml_text`: the text of an opened rule YAML, once the open file is shown to be the one its checked path names.

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// The text of `file`, opened from the canonical path of the rule YAML `check` under `folder` (canonical). `Err` is
/// the stderr text. The path is checked again by `rule_yaml_path` and must name the same file (device and inode) as
/// `file`, so what is read is the file at an in-folder canonical path after the open, and a component swapped in
/// between is refused. A file with more than one hard link is refused, before and after the read: tm cannot tell
/// where a file's other links are, so a second link is refused even when it is inside the folder too.
#[cfg(unix)]
pub(crate) fn rule_yaml_text(mut file: File, folder: &Path, check: &str) -> Result<String, String> {
    use super::rule_yaml_path;
    use std::os::unix::fs::MetadataExt;
    let cannot_read = |err: std::io::Error| format!("cannot read ast-grep rule {check}: {err}");
    let opened = file.metadata().map_err(cannot_read)?;
    let now = std::fs::metadata(rule_yaml_path(folder, check)?).map_err(cannot_read)?;
    if (opened.dev(), opened.ino()) != (now.dev(), now.ino()) {
        return Err(format!("ast-grep rule {check} changed while it was opened"));
    }
    only_link(opened.nlink(), check)?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(cannot_read)?;
    only_link(file.metadata().map_err(cannot_read)?.nlink(), check)?;
    Ok(text)
}

/// Off Unix: the text of `file` with no identity or link check, as procio's `which` does off Unix the check the
/// platform allows; the path checks of `rule_yaml_path` still ran before `file` was opened.
#[cfg(not(unix))]
pub(crate) fn rule_yaml_text(mut file: File, _folder: &Path, check: &str) -> Result<String, String> {
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|err| format!("cannot read ast-grep rule {check}: {err}"))?;
    Ok(text)
}

/// Refuses a file whose link count says some other name, anywhere, reaches it too.
#[cfg(unix)]
fn only_link(nlink: u64, check: &str) -> Result<(), String> {
    if nlink > 1 {
        return Err(format!(
            "ast-grep rule {check} has more than one hard link: tm reads a rule YAML only when its folder holds the only link"
        ));
    }
    Ok(())
}
