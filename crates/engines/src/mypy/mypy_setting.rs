//! `MypySetting`: one mypy config setting that can silence findings, as tm reads it from a config file.

/// One kept key of a mypy config section: `ignore_errors`, `ignore_missing_imports` or `disable_error_code`.
/// `section` is the label a why prints (`[mypy]`, `[mypy-pkg.*]`, `[tool.mypy]`, `[[tool.mypy.overrides]] #2`);
/// `per_module` is true for a per-module section; `flag` is the truth of a flag key (false for a code-list key);
/// `codes` is the list of a code-list key (empty for a flag key).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MypySetting {
    pub(crate) section: String,
    pub(crate) per_module: bool,
    pub(crate) key: String,
    pub(crate) flag: bool,
    pub(crate) codes: Vec<String>,
}
