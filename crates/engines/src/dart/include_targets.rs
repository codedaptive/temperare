//! `include_targets`: the files an analysis_options.yaml includes, as written.

use crate::yaml::{yaml_entries, YamlValue};

/// The targets of `text`'s top-level `include`, in order, as YAML decodes them: its scalar (`include: base.yaml`) or
/// each scalar item of its list, block or flow. A null `include` (`include:`, `~`) names none; a quoted empty one
/// (`include: ''`) names the target "", which `options_files` resolves to the including file's folder and cannot read
/// as a file (`NotAvailable`), so never clean. `text` is read as YAML
/// (`yaml_entries`), and its `Err` naming a line is passed up: an include tm cannot read is options it never checks,
/// so no doubt reads as a clean list. An `include` key anywhere else is not one, since dart reads only the top-level
/// one. Pure.
pub(crate) fn include_targets(text: &str) -> Result<Vec<String>, String> {
    Ok(yaml_entries(text)?
        .into_iter()
        .filter(|entry| entry.path == ["include"])
        .filter_map(|entry| match entry.value {
            YamlValue::Plain(target) | YamlValue::Quoted(target) => Some(target),
            YamlValue::Null | YamlValue::Nested => None,
        })
        .collect())
}
