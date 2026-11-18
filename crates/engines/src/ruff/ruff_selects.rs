//! `ruff_selects`: whether a ruff `--select` list selects a diagnostic's code.

/// True when some comma-separated part of `check` is `ALL`, or is non-empty and a string prefix of `code`. ruff
/// 0.17.4's `lint.select` is "a list of rule codes or prefixes to enable. Prefixes can specify exact rules (like
/// `F841`), entire groups (like `F`), or anything in between", and `ruff check --help` says "Comma-separated list of
/// rule codes to enable (or ALL, to enable all rules)". An empty part selects nothing. Parts are not trimmed because
/// ruff does not trim them: it refuses `E7, F4` as "Unknown rule selector ` F4`".
pub(crate) fn ruff_selects(check: &str, code: &str) -> bool {
    check.split(',').any(|part| part == "ALL" || (!part.is_empty() && code.starts_with(part)))
}
