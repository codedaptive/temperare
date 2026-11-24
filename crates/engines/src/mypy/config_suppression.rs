//! `config_suppression`: whether a mypy config file silences mypy findings of a check outside the rule file.

use super::{ini_settings, toml_settings, MypySetting};

/// Whether the config file `name` (as the why shows it) with contents `text` silences mypy findings of `check`.
/// The file is read with `toml_settings` when `name` ends in `.toml`, else with `ini_settings`. The first setting, in
/// file order, that silences gives Ok(Some(why)) naming the file, its section and its key:
/// - `ignore_errors` true, in any section;
/// - `disable_error_code` listing `check` in a per-module section (a global one is overridden by tm's
///   `--enable-error-code=<check>`, so it silences nothing);
/// - `ignore_missing_imports` true, when `check` is `import-not-found` or `import-untyped`.
///
/// Ok(None) when nothing silences; Err when the TOML does not parse, quoting the toml error.
pub(crate) fn config_suppression(name: &str, text: &str, check: &str) -> Result<Option<String>, String> {
    let settings = if name.ends_with(".toml") {
        toml_settings(text).map_err(|error| format!("mypy config {name} cannot be read: {error}"))?
    } else {
        ini_settings(text)
    };
    Ok(settings.iter().find(|setting| silences(setting, check)).map(|setting| {
        format!(
            "mypy config {name}: {} {} silences mypy findings outside the rule file; exceptions go in the rule file",
            setting.section, setting.key
        )
    }))
}

/// Whether `setting` stops mypy reporting findings of `check`.
fn silences(setting: &MypySetting, check: &str) -> bool {
    match setting.key.as_str() {
        "ignore_errors" => setting.flag,
        "disable_error_code" => setting.per_module && setting.codes.iter().any(|code| code == check),
        "ignore_missing_imports" => setting.flag && matches!(check, "import-not-found" | "import-untyped"),
        _ => false,
    }
}
