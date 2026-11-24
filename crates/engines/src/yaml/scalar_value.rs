//! `scalar_value`: one scalar event as a `YamlValue`. Tested through yaml_entries (yaml_entries_tests.rs), so no test
//! file of its own.

use super::YamlValue;
use saphyr_parser::ScalarStyle;

/// A scalar of `style` with `text`, `tagged` or not, as a value (`YamlValue` says which is which): null as the YAML
/// 1.2 core schema reads an untagged plain one, else plain when untagged and plain, else quoted.
pub(super) fn scalar_value(text: &str, style: ScalarStyle, tagged: bool) -> YamlValue {
    match style {
        ScalarStyle::Plain if !tagged && matches!(text, "" | "~" | "null" | "Null" | "NULL") => YamlValue::Null,
        ScalarStyle::Plain if !tagged => YamlValue::Plain(text.to_string()),
        _ => YamlValue::Quoted(text.to_string()),
    }
}
