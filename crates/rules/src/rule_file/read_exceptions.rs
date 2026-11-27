//! `read_exceptions`: the `[[exception]]` tables of the rule file.

use super::check_keys::check_keys;
use super::date_value::date_value;
use super::problem_at::problem_at;
use super::required_string::required_string;
use super::string_value::string_value;
use super::wrong_type::wrong_type;
use crate::{Date, Exception, Problem, RelPath};
use std::collections::BTreeSet;
use toml::de::DeTable;

const PLACE: &str = "[[exception]]";
const EXCEPTION_KEYS: [&str; 6] = ["rule", "path", "what", "reason", "approved", "expires"];
/// `required_string`'s generic fix would ask for a string; an exception's end needs the date form and its meaning.
const EXPIRES_FIX: &str = "Add `expires = \"YYYY-MM-DD\"`, the exception's last day (UTC), to this [[exception]].";

/// The exceptions of the `[[exception]]` tables without problems; `rule_ids` holds the id of every `[[rule]]` and
/// every rule id of the taken baseline sets.
pub(crate) fn read_exceptions(
    text: &str,
    root: &DeTable,
    rule_ids: &BTreeSet<String>,
    out: &mut Vec<Problem>,
) -> Vec<Exception> {
    let Some(value) = root.get("exception") else {
        return Vec::new();
    };
    let Some(tables) = value.get_ref().as_array() else {
        out.push(wrong_type(text, "exception", value, "[[exception]] tables"));
        return Vec::new();
    };
    tables
        .iter()
        .filter_map(|item| match item.get_ref().as_table() {
            Some(table) => read_exception(text, item.span().start, table, rule_ids, out),
            None => {
                out.push(wrong_type(text, "exception", item, "an [[exception]] table"));
                None
            }
        })
        .collect()
}

/// The exception in `table`, whose header starts at byte `at`; None when it has any Problem (every check still runs).
fn read_exception(
    text: &str,
    at: usize,
    table: &DeTable,
    rule_ids: &BTreeSet<String>,
    out: &mut Vec<Problem>,
) -> Option<Exception> {
    let before = out.len();
    check_keys(text, table, &EXCEPTION_KEYS, PLACE, out);
    let rule = required_string(text, table, "rule", PLACE, at, out);
    if let Some((id, id_at)) = rule.as_ref().filter(|(id, _)| !rule_ids.contains(id)) {
        out.push(problem_at(
            text,
            *id_at,
            format!("exception for unknown rule \"{id}\": no rule of this file or of its baseline sets has that id"),
            "Name the id of a rule in this file or in a set it takes in baseline, or remove the exception.",
        ));
    }
    let path = required_string(text, table, "path", PLACE, at, out);
    let what = string_value(text, table, "what", out);
    if let Some((_, what_at)) = what.as_ref().filter(|(what, _)| what.trim().is_empty()) {
        out.push(problem_at(
            text,
            *what_at,
            "exception `what` is empty".to_string(),
            "empty what; omit the key to match every finding of the rule at this path",
        ));
    }
    let reason = required_string(text, table, "reason", PLACE, at, out);
    let approved = required_string(text, table, "approved", PLACE, at, out);
    let expires = required_date(text, table, at, out);
    let (Some((rule, _)), Some((path, _)), Some((reason, _)), Some((approved, _)), Some(expires)) =
        (rule, path, reason, approved, expires)
    else {
        return None;
    };
    if out.len() > before {
        return None;
    }
    Some(Exception {
        rule,
        path: RelPath::new(&path),
        what: what.map(|(what, _)| what),
        expires: Some(expires),
        reason,
        approved,
    })
}

/// The `expires` date of the table whose header starts at byte `at`; absent is a Problem at `at`.
fn required_date(text: &str, table: &DeTable, at: usize, out: &mut Vec<Problem>) -> Option<Date> {
    if table.get("expires").is_none() {
        out.push(problem_at(text, at, format!("{PLACE} is missing `expires`"), EXPIRES_FIX));
        return None;
    }
    date_value(text, table, "expires", out)
}
