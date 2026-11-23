//! `toml_settings`: the mypy settings that can silence findings in a pyproject.toml.

use super::{config_true, MypySetting};
use toml::de::{DeTable, DeValue};
use toml::Spanned;

/// The settings of `text` read as TOML through the `toml` crate, in file order (by where each key is written):
/// - `[tool.mypy]` is global, labelled `[tool.mypy]`; each table of the array `tool.mypy.overrides` is per-module,
///   labelled `[[tool.mypy.overrides]] #<n>`, n counted from 1;
/// - kept keys, exact as mypy reads them (no lowercasing, no dashes): `ignore_errors` and `ignore_missing_imports`,
///   a flag being a TOML bool or a string read by `config_true` (any other type is not kept); and `disable_error_code`, its codes an array of strings or one string split on `,`,
///   each trimmed, empties dropped.
///
/// Err with the toml error text when `text` is not TOML; Ok(empty) when it has no `[tool.mypy]` table.
pub(crate) fn toml_settings(text: &str) -> Result<Vec<MypySetting>, String> {
    let document = DeTable::parse(text).map_err(|error| error.to_string())?;
    let Some(mypy) = table_at(document.get_ref(), "tool").and_then(|tool| table_at(tool, "mypy")) else {
        return Ok(Vec::new());
    };
    let mut found = settings(mypy, "[tool.mypy]", false);
    if let Some(DeValue::Array(overrides)) = mypy.get("overrides").map(Spanned::get_ref) {
        for (at, item) in overrides.iter().enumerate() {
            if let DeValue::Table(table) = item.get_ref() {
                found.extend(settings(table, &format!("[[tool.mypy.overrides]] #{}", at + 1), true));
            }
        }
    }
    found.sort_by_key(|(start, _)| *start);
    Ok(found.into_iter().map(|(_, setting)| setting).collect())
}

/// The table under `key` in `table`, when that value is a table.
fn table_at<'t, 'i>(table: &'t DeTable<'i>, key: &str) -> Option<&'t DeTable<'i>> {
    match table.get(key).map(Spanned::get_ref) {
        Some(DeValue::Table(inner)) => Some(inner),
        _ => None,
    }
}

/// The kept keys of one mypy table, each with the byte offset where its key is written.
fn settings(table: &DeTable<'_>, section: &str, per_module: bool) -> Vec<(usize, MypySetting)> {
    table
        .iter()
        .filter_map(|(key, value)| {
            let (flag, codes) = read(key.get_ref(), value.get_ref())?;
            let setting =
                MypySetting { section: section.to_string(), per_module, key: key.get_ref().to_string(), flag, codes };
            Some((key.span().start, setting))
        })
        .collect()
}

/// The flag and codes of a kept key's value; None for another key, or a value of a type the key does not take.
fn read(key: &str, value: &DeValue<'_>) -> Option<(bool, Vec<String>)> {
    match (key, value) {
        ("ignore_errors" | "ignore_missing_imports", DeValue::Boolean(flag)) => Some((*flag, Vec::new())),
        ("ignore_errors" | "ignore_missing_imports", DeValue::String(word)) => Some((config_true(word), Vec::new())),
        ("disable_error_code", DeValue::String(list)) => Some((false, codes(list.split(',')))),
        ("disable_error_code", DeValue::Array(items)) => Some((
            false,
            codes(items.iter().filter_map(|item| match item.get_ref() {
                DeValue::String(code) => Some(code.as_ref()),
                _ => None,
            })),
        )),
        _ => None,
    }
}

/// `items` trimmed, empties dropped.
fn codes<'a>(items: impl Iterator<Item = &'a str>) -> Vec<String> {
    items.map(str::trim).filter(|code| !code.is_empty()).map(String::from).collect()
}
