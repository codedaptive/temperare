//! `bool_value`: an optional boolean key of a rule-file table.

use super::wrong_type::wrong_type;
use crate::Problem;
use toml::de::DeTable;

/// The boolean under `key` and its byte offset. None when the key is absent, or when it is not a boolean (a Problem).
pub(crate) fn bool_value(text: &str, table: &DeTable, key: &str, out: &mut Vec<Problem>) -> Option<(bool, usize)> {
    let value = table.get(key)?;
    match value.get_ref().as_bool() {
        Some(b) => Some((b, value.span().start)),
        None => {
            out.push(wrong_type(text, key, value, "a boolean"));
            None
        }
    }
}
