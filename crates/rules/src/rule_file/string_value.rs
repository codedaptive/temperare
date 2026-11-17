//! `string_value`: an optional string key of a rule-file table.

use super::wrong_type::wrong_type;
use crate::Problem;
use toml::de::DeTable;

/// The string under `key` and its byte offset. None when the key is absent, or when it is not a string (a Problem).
pub(crate) fn string_value(text: &str, table: &DeTable, key: &str, out: &mut Vec<Problem>) -> Option<(String, usize)> {
    let value = table.get(key)?;
    match value.get_ref().as_str() {
        Some(s) => Some((s.to_string(), value.span().start)),
        None => {
            out.push(wrong_type(text, key, value, "a string"));
            None
        }
    }
}
