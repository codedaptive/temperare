//! `ini_settings`: the mypy settings that can silence findings in an INI config (mypy.ini, .mypy.ini, setup.cfg).

use super::{config_true, MypySetting};

/// The settings of `text` read as Python's configparser (its defaults) and mypy read an INI config, in file order:
/// - sections: `[mypy]` is global, a section whose name starts `mypy-` is per-module, and `[DEFAULT]` counts as
///   per-module: configparser copies its keys into every section, so they land in any per-module section too. Any
///   other section is skipped. A header is a line opening with `[`, its name running to the line's last `]`; names
///   are case-sensitive;
/// - blank lines and full-line comments (first non-space char `#` or `;`) are skipped; there are no inline comments;
/// - `key = value` or `key: value`, split at the first `=` or `:`, the key trimmed and lowercased, the value trimmed;
///   a line indented deeper than the key's line continues its value, joined with a newline;
/// - kept keys: `ignore_errors` and `ignore_missing_imports`, whose flag is true when the value, lowercased, is one
///   of 1, yes, true, on (`config_true`), and `disable_error_code`, whose codes are the value split on
///   `,` and newlines, each trimmed, empties dropped. A dashed key (`ignore-errors`) is not mypy's and is not kept.
///
/// A line configparser would reject (a key before any section, a line that is neither header nor key) is skipped.
pub(crate) fn ini_settings(text: &str) -> Vec<MypySetting> {
    entries(text).into_iter().filter_map(|(section, key, value)| setting(section, &key, &value)).collect()
}

/// Each key of `text` as (section name, lowercased key, value with its continuation lines).
fn entries(text: &str) -> Vec<(&str, String, String)> {
    let mut entries: Vec<(&str, String, String)> = Vec::new();
    let mut section = None;
    let mut key_open = false;
    let mut level = 0;
    for line in text.lines() {
        let value = line.trim();
        if value.is_empty() || value.starts_with(['#', ';']) {
            continue;
        }
        let indent = line.len() - line.trim_start().len();
        if let (true, true, Some(last)) = (key_open, indent > level, entries.last_mut()) {
            last.2.push('\n');
            last.2.push_str(value);
            continue;
        }
        level = indent;
        if let Some(name) = header(value) {
            section = Some(name);
            key_open = false;
            continue;
        }
        let (Some(name), Some((key, rest))) = (section, value.split_once(['=', ':'])) else {
            continue;
        };
        entries.push((name, key.trim().to_lowercase(), rest.trim().to_string()));
        key_open = true;
    }
    entries
}

/// The section name of a header line: from after its opening `[` to its last `]`, not empty.
fn header(value: &str) -> Option<&str> {
    let inner = value.strip_prefix('[')?;
    inner.rfind(']').map(|end| &inner[..end]).filter(|name| !name.is_empty())
}

/// The setting for `key = value` in `section`, when the section is mypy's and the key is kept.
fn setting(section: &str, key: &str, value: &str) -> Option<MypySetting> {
    let per_module = match section {
        "mypy" => false,
        "DEFAULT" => true,
        _ if section.starts_with("mypy-") => true,
        _ => return None,
    };
    let (flag, codes) = match key {
        "ignore_errors" | "ignore_missing_imports" => (config_true(value), Vec::new()),
        "disable_error_code" => {
            (false, value.split([',', '\n']).map(str::trim).filter(|code| !code.is_empty()).map(String::from).collect())
        }
        _ => return None,
    };
    Some(MypySetting { section: format!("[{section}]"), per_module, key: key.to_string(), flag, codes })
}
