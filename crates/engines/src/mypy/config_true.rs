//! `config_true`: the truth of a mypy config flag's text.

/// True when `value`, lowercased, is one of 1, yes, true, on: configparser's truth table, which mypy uses for a flag
/// in an INI config and for a string flag in a pyproject.toml. Any other text is false.
pub(crate) fn config_true(value: &str) -> bool {
    matches!(value.to_lowercase().as_str(), "1" | "yes" | "true" | "on")
}
