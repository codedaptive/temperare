//! `date_value`: an optional date key of a rule-file table.

use super::problem_at::problem_at;
use super::string_value::string_value;
use crate::{Date, Problem};
use toml::de::DeTable;

/// The date under `key`, written "YYYY-MM-DD". None when the key is absent; None and a Problem at the value when it
/// is not a string, or is a string that is not a real date in that form.
pub(crate) fn date_value(text: &str, table: &DeTable, key: &str, out: &mut Vec<Problem>) -> Option<Date> {
    let (s, at) = string_value(text, table, key, out)?;
    let date = Date::parse(&s);
    if date.is_none() {
        out.push(problem_at(
            text,
            at,
            format!("`{key}` must be a real date written YYYY-MM-DD, not \"{s}\""),
            &format!("Write `{key}` as a real calendar date, \"YYYY-MM-DD\"."),
        ));
    }
    date
}
