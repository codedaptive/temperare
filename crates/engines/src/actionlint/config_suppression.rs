//! `config_suppression`: an `ignore` in actionlint's config that cctao cannot turn off.

use crate::cannot_turn_off::CANNOT_TURN_OFF;
use crate::types::Stop;
use crate::yaml::yaml_entries;
use std::path::Path;

/// `Failed` naming `path` and the line when `text`, the config cctao passes actionlint, holds an entry whose key path
/// holds the key `ignore`, or when cctao cannot read it; None otherwise. actionlint drops every finding an `ignore` regex
/// matches and no option turns that off, so the rule gives no answer; exceptions go in the rule file.
///
/// The text is read as YAML (`yaml_entries`), every style alike, escapes decoded and aliases replayed, so a quoted or
/// escaped key (`"\x69gnore"`, which actionlint honours) and an `ignore` reached through an alias are found. The key is
/// matched in its exact case: actionlint does not honour `Ignore`. What `yaml_entries` cannot read (a duplicate key,
/// a second document, a merge key `<<`, which actionlint applies) is `Failed` with its why, never a clean read.
pub(crate) fn config_suppression(path: &Path, text: &str) -> Option<Stop> {
    let why = match yaml_entries(text) {
        // `why` starts `line N: `; the line stays first, as in the message below.
        Err(why) => format!("{}; put exceptions in the rule file", why.replacen(": ", ": cctao cannot read it: ", 1)),
        Ok(entries) => {
            let entry = entries.iter().find(|entry| entry.path.iter().any(|key| key == "ignore"))?;
            format!(
                "line {}: `{}`: actionlint drops findings matching it and {CANNOT_TURN_OFF}",
                entry.line,
                entry.path.join(": ")
            )
        }
    };
    Some(Stop::Failed { code: None, stderr: format!("{}: {why}", path.display()) })
}
