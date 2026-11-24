//! `yaml_settings`: a `.markdownlint.yaml` or `.yml` read into its top-level keys, in order, each with whether it turns
//! rules on.

use super::json_settings;
use crate::yaml::{yaml_entries, YamlEntry, YamlValue};

/// The top-level keys of `text`, in file order, each with what markdownlint 0.42.2's `getEffectiveConfig` makes of its
/// value, as `json_settings` says, the value read as js-yaml 5.5.0 (the reader markdownlint-cli2 0.24.2 bundles, YAML
/// 1.2 core schema) gives it to JavaScript. Measured: plain `false`/`False`/`FALSE`, the null forms, a number equal to
/// zero (`0`, `-0`, `+0`, `00`, `0.0`, `.0`, `0.`, `0e0`, `0x0`, `0o0` and their longer forms) and `.nan`/`.NaN`/`.NAN`
/// are falsy; any other plain scalar is truthy, `yes`, `no`, `on` and `off` included (strings in YAML 1.2). A mapping
/// under a key is on unless its `enabled` child is falsy, read the same way; a sequence is on.
///
/// markdownlint-cli2 reads a config with JSONC first, then TOML, then YAML, keeping the first that parses, so these are
/// `Err` rather than a reading that may differ from its own (measured: `MD013 = false` is read as TOML, and
/// `{"MD013": /* x */ false}` as JSONC, which YAML would read as the string `/* x */ false`):
/// - text whose first character is `{` or `[`, unless it is plain JSON, which is read by `json_settings`;
/// - text with content (beyond blank lines, `#` comments and document markers) whose top level is not a mapping;
/// - a top-level key holding `=`, TOML's `key = value` read by YAML as a key: no rule name holds one.
///
/// A quoted or tagged scalar comes from `yaml_entries` as `Quoted` with its tag dropped, so `"false"` (a truthy string)
/// and `!!bool false` look the same: an empty one is falsy, one whose text is a falsy spelling above is an `Err` (it
/// over-refuses a quoted `"false"`, which is safe), and any other is truthy. `yaml_entries`'s `Err`s pass through: a
/// key that repeats in a mapping among them, which js-yaml rejects too.
pub(crate) fn yaml_settings(text: &str) -> Result<Vec<(String, bool)>, String> {
    if text.trim_start_matches('\u{feff}').trim_start().starts_with(['{', '[']) {
        return json_settings(text).map_err(|err| {
            format!("flow-style text that is not plain JSON ({err}): markdownlint-cli2 reads it as JSONC first")
        });
    }
    let entries = yaml_entries(text)?;
    let mapping = entries.first().map_or(!has_content(text), |entry| !entry.path.is_empty());
    if !mapping {
        return Err("its top level is not a mapping: markdownlint-cli2 would read it as TOML or JSONC first".into());
    }
    let mut settings = Vec::new();
    for (at, entry) in entries.iter().enumerate() {
        let ([key], false) = (entry.path.as_slice(), entry.item) else { continue };
        if key.contains('=') {
            let line = entry.line;
            return Err(format!("line {line}: the key `{key}` holds `=`, which markdownlint-cli2 may read as TOML"));
        }
        let on = match &entry.value {
            YamlValue::Nested if key.eq_ignore_ascii_case("default") => true,
            YamlValue::Nested => nested_on(key, &entries[at + 1..])?,
            value => truthy(value, entry.line)?,
        };
        settings.push((key.clone(), on));
    }
    Ok(settings)
}

/// Whether a mapping or sequence under the top-level `key` is on, `after` being the entries that follow it: a
/// sequence (its first entry an item at `key`'s own path) is on; a mapping is on unless its `enabled` child is falsy.
fn nested_on(key: &str, after: &[YamlEntry]) -> Result<bool, String> {
    if after.first().is_some_and(|entry| entry.item && entry.path.len() == 1) {
        return Ok(true);
    }
    let mut children = after.iter().take_while(|entry| entry.path.len() > 1);
    match children.find(|entry| !entry.item && entry.path == [key, "enabled"]) {
        Some(YamlEntry { value: YamlValue::Nested, .. }) | None => Ok(true),
        Some(entry) => truthy(&entry.value, entry.line),
    }
}

/// A scalar's truthiness, as the doc of `yaml_settings` says; `line` names it in an `Err`.
fn truthy(value: &YamlValue, line: usize) -> Result<bool, String> {
    match value {
        YamlValue::Null => Ok(false),
        YamlValue::Plain(text) => Ok(!falsy_spelling(text)),
        YamlValue::Quoted(text) if text.is_empty() => Ok(false),
        YamlValue::Quoted(text) if falsy_spelling(text) || matches!(text.as_str(), "~" | "null" | "Null" | "NULL") => {
            Err(format!("line {line}: a quoted or tagged `{text}`, whose tag tm cannot see: it may be falsy"))
        }
        _ => Ok(true),
    }
}

/// Whether `text`, read as an untagged plain scalar, is falsy apart from the null forms: a false spelling, a number
/// equal to zero in a core-schema form, or a NaN.
fn falsy_spelling(text: &str) -> bool {
    matches!(text, "false" | "False" | "FALSE" | ".nan" | ".NaN" | ".NAN") || zero(text)
}

/// Whether `text` is a core-schema int (`[-+]?[0-9]+`, `0o[0-7]+`, `0x[0-9a-fA-F]+`) or float
/// (`[-+]?(\.[0-9]+|[0-9]+(\.[0-9]*)?)([eE][-+]?[0-9]+)?`) whose digits before any exponent are all `0`.
fn zero(text: &str) -> bool {
    let zeros = |part: &str| part.bytes().all(|byte| byte == b'0');
    let digits = |part: &str| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit());
    if let Some(radix) = text.strip_prefix("0o").or_else(|| text.strip_prefix("0x")) {
        return !radix.is_empty() && zeros(radix);
    }
    let body = text.strip_prefix(['-', '+']).unwrap_or(text);
    let (mantissa, exponent) = body.split_once(['e', 'E']).unwrap_or((body, "0"));
    let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let shaped =
        (digits(whole) || (whole.is_empty() && !fraction.is_empty())) && (fraction.is_empty() || digits(fraction));
    shaped && digits(exponent.strip_prefix(['-', '+']).unwrap_or(exponent)) && zeros(whole) && zeros(fraction)
}

/// Whether `text` holds a line other than blank lines, `#` comments and the document markers `---` and `...`.
fn has_content(text: &str) -> bool {
    text.lines().map(str::trim).any(|line| !(line.is_empty() || line.starts_with('#') || matches!(line, "---" | "...")))
}
