//! `required_string`: a string key a rule-file table must have.

use super::problem_at::problem_at;
use super::string_value::string_value;
use crate::Problem;
use toml::de::DeTable;

/// The string under `key` and its byte offset. Absent is a Problem at `at`, the start of the table `place`
/// (e.g. "[[rule]]"); a value that is not a string is a Problem at the value.
pub(crate) fn required_string(
    text: &str,
    table: &DeTable,
    key: &str,
    place: &str,
    at: usize,
    out: &mut Vec<Problem>,
) -> Option<(String, usize)> {
    if table.get(key).is_none() {
        out.push(problem_at(
            text,
            at,
            format!("{place} is missing `{key}`"),
            &format!("Add `{key} = \"...\"` to this {place}."),
        ));
        return None;
    }
    string_value(text, table, key, out)
}
