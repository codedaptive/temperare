//! `read_overrides`: the `[[override]]` tables of the rule file.

use super::bool_value::bool_value;
use super::check_globs::check_globs;
use super::check_keys::check_keys;
use super::check_not_option::check_not_option;
use super::override_entry::OverrideEntry;
use super::problem_at::problem_at;
use super::required_string::required_string;
use super::severity_value::severity_value;
use super::string_list::string_list;
use super::wrong_type::wrong_type;
use crate::Problem;
use std::collections::BTreeSet;
use toml::de::DeTable;

const PLACE: &str = "[[override]]";
const OVERRIDE_KEYS: [&str; 6] = ["rule", "severity", "off", "paths", "exclude", "params"];

/// The overrides of the `[[override]]` tables whose `rule` is a string, in file order, each with its bad fields read
/// as not given (their Problems are in `out`), so the caller can still check every rule id. A second `[[override]]`
/// for the same rule is a Problem at its `rule` string and is left out. Whether the rule is in a taken set is not
/// checked here.
pub(crate) fn read_overrides(text: &str, root: &DeTable, out: &mut Vec<Problem>) -> Vec<OverrideEntry> {
    let Some(value) = root.get("override") else {
        return Vec::new();
    };
    let Some(tables) = value.get_ref().as_array() else {
        out.push(wrong_type(text, "override", value, "[[override]] tables"));
        return Vec::new();
    };
    // One plain parse of the whole text gives every override's `params` as a toml::Table directly, as read_rules
    // does for `[[rule]]` params.
    let doc: toml::Table = toml::from_str(text).unwrap_or_default();
    let mut ids = BTreeSet::new();
    let mut entries = Vec::new();
    for (i, item) in tables.iter().enumerate() {
        let Some(table) = item.get_ref().as_table() else {
            out.push(wrong_type(text, "override", item, "an [[override]] table"));
            continue;
        };
        let second = is_second(text, table, &mut ids, out);
        let entry = read_override(text, item.span().start, table, params(&doc, i), out);
        entries.extend(entry.filter(|_| !second));
    }
    entries
}

/// The override in `table`, whose header starts at byte `at`; `params` is its `params` when that is a table. Every
/// check runs and each bad field is read as not given: severity None, off false, paths and exclude holding only the
/// globs that compiled (None when not an array), params None when not a table or when check_not_option finds a value
/// that begins with '-' (a `params.allow` value; `check` and `config` are unknown keys here). None only when `rule` is
/// missing or not a string.
fn read_override(
    text: &str,
    at: usize,
    table: &DeTable,
    params: Option<toml::Table>,
    out: &mut Vec<Problem>,
) -> Option<OverrideEntry> {
    check_keys(text, table, &OVERRIDE_KEYS, PLACE, out);
    let rule = required_string(text, table, "rule", PLACE, at, out);
    // severity_value reads an absent severity as Error; here absent means the baseline's severity stands.
    let severity = table.get("severity").and_then(|_| severity_value(text, table, out));
    let off = bool_value(text, table, "off", out).is_some_and(|(off, _)| off);
    let paths = globs(text, table, "paths", out);
    let exclude = globs(text, table, "exclude", out);
    if let Some(value) = table.get("params").filter(|v| !v.get_ref().is_table()) {
        out.push(wrong_type(text, "params", value, "a table"));
    }
    let before = out.len();
    check_not_option(text, table, out);
    let params = params.filter(|_| out.len() == before);
    let (rule, rule_at) = rule?;
    Some(OverrideEntry { rule, at: rule_at, severity, off, paths, exclude, params })
}

/// The globs under `key` that compile; None when the key is absent or not an array.
fn globs(text: &str, table: &DeTable, key: &str, out: &mut Vec<Problem>) -> Option<Vec<String>> {
    let given = table.get(key)?.get_ref().is_array();
    let compiled = check_globs(text, &string_list(text, table, key, out), out);
    given.then_some(compiled)
}

/// Records the table's `rule` in `ids`; true, with a Problem at the string, when it was there already.
fn is_second(text: &str, table: &DeTable, ids: &mut BTreeSet<String>, out: &mut Vec<Problem>) -> bool {
    let Some(value) = table.get("rule") else {
        return false;
    };
    let Some(id) = value.get_ref().as_str() else {
        return false;
    };
    if ids.insert(id.to_string()) {
        return false;
    }
    out.push(problem_at(
        text,
        value.span().start,
        format!("second [[override]] for \"{id}\""),
        "Put every change to one rule in a single [[override]].",
    ));
    true
}

/// The `params` table of the `i`th `[[override]]` in `doc`; None when absent or not a table.
fn params(doc: &toml::Table, i: usize) -> Option<toml::Table> {
    doc.get("override")
        .and_then(|overrides| overrides.get(i))
        .and_then(|entry| entry.get("params"))
        .and_then(|params| params.as_table())
        .cloned()
}
