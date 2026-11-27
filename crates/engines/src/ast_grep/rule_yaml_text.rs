//! `rule_yaml_text`: the text of an opened rule YAML, on Unix once its link count shows no other name reaches it.

use std::fs::File;
use std::io::Read;

/// The text of `file`, the rule YAML `check` as `open_in_folder` opened it. `Err` is the stderr text. The open
/// file's own link count is read from the handle before and after the read: a count of 0 means the opened file was
/// unlinked or renamed over since the open, and is refused as changed while it was opened; a count above 1 is
/// refused because cctao cannot tell where a file's other links are, so a second link is refused even when it is
/// inside the folder too.
#[cfg(unix)]
pub(crate) fn rule_yaml_text(mut file: File, check: &str) -> Result<String, String> {
    use std::os::unix::fs::MetadataExt;
    let cannot_read = |err: std::io::Error| format!("cannot read ast-grep rule {check}: {err}");
    only_link(file.metadata().map_err(cannot_read)?.nlink(), check)?;
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(cannot_read)?;
    only_link(file.metadata().map_err(cannot_read)?.nlink(), check)?;
    Ok(text)
}

/// Off Unix: the text of `file` with no link check, as procio's `which` does off Unix the check the platform
/// allows; the path checks of `rule_yaml_path` still ran before `file` was opened.
#[cfg(not(unix))]
pub(crate) fn rule_yaml_text(mut file: File, check: &str) -> Result<String, String> {
    let mut text = String::new();
    file.read_to_string(&mut text).map_err(|err| format!("cannot read ast-grep rule {check}: {err}"))?;
    Ok(text)
}

/// Refuses a file no name reaches any more (0), or one that some other name, anywhere, reaches too (above 1).
#[cfg(unix)]
fn only_link(nlink: u64, check: &str) -> Result<(), String> {
    match nlink {
        0 => Err(format!("ast-grep rule {check} changed while it was opened")),
        1 => Ok(()),
        _ => Err(format!(
            "ast-grep rule {check} has more than one hard link: cctao reads a rule YAML only when its folder holds the only link"
        )),
    }
}
