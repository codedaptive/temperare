//! `check_not_option`: a rule's `check`, `config` and `params.allow` values must not read as engine options.

use super::problem_at::problem_at;
use crate::Problem;
use toml::Spanned;
use toml::de::{DeTable, DeValue};

const FIX: &str = "A check, config or allow value is never an engine option: remove the leading '-'.";

/// A Problem at each string value of `check`, `config` and the elements of `params.allow` in the raw `[[rule]]`
/// `table` that starts with '-': the engine would take it as an option. A `check` or `config` of another type and a
/// `params` that is not a table are left to the type checks; an `allow` that is not a list, or a list element that is
/// not a string, is the clippy engine's to refuse at run time; other `params` keys are the engine's.
pub(crate) fn check_not_option(text: &str, table: &DeTable, out: &mut Vec<Problem>) {
    for key in ["check", "config"] {
        if let Some(value) = table.get(key) {
            flag_dashed(text, key, value, out);
        }
    }
    let allow = table
        .get("params")
        .and_then(|params| params.get_ref().as_table()?.get("allow"))
        .and_then(|allow| allow.get_ref().as_array());
    for item in allow.into_iter().flatten() {
        flag_dashed(text, "params.allow", item, out);
    }
}

/// A Problem at `value` when it is a string that starts with '-'.
fn flag_dashed(text: &str, key: &str, value: &Spanned<DeValue>, out: &mut Vec<Problem>) {
    let Some(s) = value.get_ref().as_str().filter(|s| s.starts_with('-')) else {
        return;
    };
    out.push(problem_at(text, value.span().start, format!("{key} \"{s}\" begins with '-'"), FIX));
}
