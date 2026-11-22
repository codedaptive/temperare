//! `allowlist_key`: where a parsed gitleaks config first declares an allowlist.

use toml::{Table, Value};

/// gitleaks' own table names, the only key segments printed as written: any other segment may be config text that
/// holds a secret, so it prints as `<key>`.
const TABLE_NAMES: [&str; 4] = ["rules", "extend", "allowlist", "allowlists"];

/// The path of the first key segment, depth-first in `table`'s key order, named `allowlist` or `allowlists` in any
/// ASCII case, searched through nested tables and arrays: dotted, with array indexes in brackets
/// (`rules[0].allowlist`). gitleaks reads its keys case-insensitively and splits a quoted key on '.', so each key is
/// cut into segments on '.' and each segment judged on its own (`"allowlist.paths"` gives `allowlist`). The matched
/// segment ends the path, whatever its value; every segment on the path prints as written only when it is one of
/// `TABLE_NAMES`, else as `<key>`. The text inside a string value is never searched. None when there is no such
/// segment.
pub(crate) fn allowlist_key(table: &Table) -> Option<String> {
    table.iter().find_map(|(key, value)| in_entry(key, value))
}

/// The path from `key` down to an allowlist segment in it or below its value.
fn in_entry(key: &str, value: &Value) -> Option<String> {
    let segments: Vec<&str> = key.split('.').collect();
    let matched = segments.iter().position(|segment| {
        segment.eq_ignore_ascii_case("allowlist") || segment.eq_ignore_ascii_case("allowlists")
    });
    let path = segments
        .iter()
        .take(matched.map_or(segments.len(), |at| at + 1))
        .map(|segment| shown(segment))
        .collect::<Vec<_>>()
        .join(".");
    match matched {
        Some(_) => Some(path),
        None => in_value(value).map(|rest| format!("{path}{rest}")),
    }
}

/// `segment` as written when it is one of `TABLE_NAMES` in any ASCII case, else `<key>`.
fn shown(segment: &str) -> &str {
    if TABLE_NAMES.iter().any(|name| segment.eq_ignore_ascii_case(name)) { segment } else { "<key>" }
}

/// The rest of the path below a value: `.<key>...` into a table, `[<index>]...` into an array.
fn in_value(value: &Value) -> Option<String> {
    match value {
        Value::Table(table) => allowlist_key(table).map(|path| format!(".{path}")),
        Value::Array(items) => {
            items.iter().enumerate().find_map(|(at, item)| in_value(item).map(|rest| format!("[{at}]{rest}")))
        }
        _ => None,
    }
}
