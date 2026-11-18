//! `read_rule_yaml`: the text of a rule's ast-grep YAML, read from the one file that is inside the rule file's folder.

use super::{rule_yaml_path, rule_yaml_text};
use std::path::Path;

/// The text of the rule YAML `check` under `folder` (canonical). `Err` is the stderr text. The path is checked by
/// `rule_yaml_path` (its refusals unchanged) and opened once; `rule_yaml_text` then checks the open file against
/// the path checked again (on Unix: the same file, and its folder holds its only hard link) and reads it.
pub(crate) fn read_rule_yaml(folder: &Path, check: &str) -> Result<String, String> {
    let file = std::fs::File::open(rule_yaml_path(folder, check)?)
        .map_err(|err| format!("cannot read ast-grep rule {check}: {err}"))?;
    rule_yaml_text(file, folder, check)
}
