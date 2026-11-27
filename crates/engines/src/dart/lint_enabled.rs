//! `lint_enabled`: whether an analysis options chain turns a lint on.

use crate::yaml::{yaml_entries, YamlEntry, YamlValue};
use std::path::PathBuf;

/// The plain scalars YAML reads as the boolean true; quoted, each is a string.
const TRUE: [&str; 3] = ["true", "True", "TRUE"];

/// Whether `chain` (each file's path and text, as `options_files` gives them) enables `lint`, each file read as YAML
/// (`yaml_entries`), block or flow. Enabled only on positive evidence, an item `<lint>` (plain or quoted) under
/// `linter: rules:` or `linter: rules: <lint>: true` (plain, as YAML reads a boolean), in some file, and no doubt in
/// any: a `linter: rules: <lint>` with any other value (`false`, a quoted `true`, none, a mapping or list) and a file
/// YAML cannot read each make it not enabled, wherever in the chain and whichever file includes which. Dart runs a
/// lint only where the options enable it, so a lint read as enabled when it is not would give a clean read for a
/// check that never ran.
pub(crate) fn lint_enabled(chain: &[(PathBuf, String)], lint: &str) -> bool {
    let mut enabled = false;
    for (_, text) in chain {
        let Ok(entries) = yaml_entries(text) else {
            return false;
        };
        for entry in entries {
            match said(&entry, lint) {
                Some(true) => enabled = true,
                Some(false) => return false,
                None => {}
            }
        }
    }
    enabled
}

/// What `entry` says of `lint`: `Some(true)` enables it, `Some(false)` turns it off or casts doubt on it, None says
/// nothing of it.
fn said(entry: &YamlEntry, lint: &str) -> Option<bool> {
    match entry.path.as_slice() {
        [linter, rules] if linter == "linter" && rules == "rules" => {
            let named = matches!(&entry.value, YamlValue::Plain(item) | YamlValue::Quoted(item) if item == lint);
            (entry.item && named).then_some(true)
        }
        [linter, rules, name] if linter == "linter" && rules == "rules" && name == lint => {
            Some(matches!(&entry.value, YamlValue::Plain(text) if TRUE.contains(&text.as_str())))
        }
        _ => None,
    }
}
