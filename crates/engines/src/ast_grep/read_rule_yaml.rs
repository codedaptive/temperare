//! `read_rule_yaml`: the text of a rule's ast-grep YAML, read from the one file that is inside the rule file's folder.

use super::{open_in_folder, rule_yaml_path, rule_yaml_text};
use std::path::Path;

/// The text of the rule YAML `check` under `folder` (canonical). `Err` is the stderr text. `rule_yaml_path` refuses
/// an absolute `check` and a path that resolves outside `folder`; `open_in_folder` then opens that path from
/// `folder`'s handle down without following a symlink (on Unix), so the file opened is inside the folder whatever the
/// path resolves to later; `rule_yaml_text` reads the opened file, refusing it when it has more than one hard link or
/// none (on Unix).
pub(crate) fn read_rule_yaml(folder: &Path, check: &str) -> Result<String, String> {
    let path = rule_yaml_path(folder, check)?;
    let file = open_in_folder(folder, &path, check)?;
    rule_yaml_text(file, check)
}
