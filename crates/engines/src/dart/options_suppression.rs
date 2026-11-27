//! `options_suppression`: a suppression in an analysis options chain that cctao cannot turn off.

use crate::cannot_turn_off::CANNOT_TURN_OFF;
use crate::types::Stop;
use crate::yaml::{yaml_entries, YamlValue};
use std::path::PathBuf;

/// `Failed` for the first file of `chain` (each file's path and text, as `options_files` gives them) that hides
/// findings or that cctao cannot read, naming the file and the line. Dart reads these from the options and cctao cannot
/// tell it not to, so the rule gives no answer; exceptions go in the rule file. None when no file has one.
///
/// Each file is read as YAML (`yaml_entries`), every style alike, escapes decoded and aliases replayed; a file it
/// cannot read is `Failed` with its why. An entry hides findings when its key path holds a key `exclude` (in any
/// case), which covers `analyzer: exclude:` and every path under it, or its scalar, a value or an item, plain or
/// quoted, is `ignore` (in any case), which covers `analyzer: errors: <code>: ignore`. The wider match costs only a
/// stopped rule where nothing is hidden (`- ignore` as a lint); a suppression missed is findings hidden behind a clean
/// read.
pub(crate) fn options_suppression(chain: &[(PathBuf, String)]) -> Option<Stop> {
    chain.iter().find_map(|(path, text)| {
        hiding(text).map(|why| Stop::Failed { code: None, stderr: format!("{}: {why}", path.display()) })
    })
}

/// Why `text` stops the rule: its first entry that hides findings, or the line `yaml_entries` cannot read.
fn hiding(text: &str) -> Option<String> {
    let entries = match yaml_entries(text) {
        Ok(entries) => entries,
        // `why` starts `line N: `; the line stays first, as in every message here.
        Err(why) => {
            let why = why.replacen(": ", ": cctao cannot read it: ", 1);
            return Some(format!("{why}; put exceptions in the rule file"));
        }
    };
    entries.iter().find_map(|entry| {
        let (scalar, after) = match &entry.value {
            YamlValue::Plain(text) | YamlValue::Quoted(text) if entry.item => (text.as_str(), ": - "),
            YamlValue::Plain(text) | YamlValue::Quoted(text) => (text.as_str(), ": "),
            YamlValue::Null | YamlValue::Nested => ("", ""),
        };
        let hides =
            entry.path.iter().any(|key| key.eq_ignore_ascii_case("exclude")) || scalar.eq_ignore_ascii_case("ignore");
        hides.then(|| {
            format!(
                "line {}: `{}{after}{scalar}`: dart hides findings there and {CANNOT_TURN_OFF}",
                entry.line,
                entry.path.join(": ")
            )
        })
    })
}
