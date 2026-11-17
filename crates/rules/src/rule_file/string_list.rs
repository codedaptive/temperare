//! `string_list`: an optional array-of-strings key of a rule-file table.

use super::wrong_type::wrong_type;
use crate::Problem;
use toml::de::DeTable;

/// Each string of the array under `key` with its byte offset; absent is empty. A value that is not an array, and
/// each element that is not a string, is a Problem.
pub(crate) fn string_list(text: &str, table: &DeTable, key: &str, out: &mut Vec<Problem>) -> Vec<(String, usize)> {
    let Some(value) = table.get(key) else {
        return Vec::new();
    };
    let Some(items) = value.get_ref().as_array() else {
        out.push(wrong_type(text, key, value, "an array of strings"));
        return Vec::new();
    };
    items
        .iter()
        .filter_map(|item| match item.get_ref().as_str() {
            Some(s) => Some((s.to_string(), item.span().start)),
            None => {
                out.push(wrong_type(text, key, item, "an array of strings"));
                None
            }
        })
        .collect()
}
