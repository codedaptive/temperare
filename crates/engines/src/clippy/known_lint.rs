//! `known_lint`: whether a clippy rule's check is a lint the toolchain lists.

/// Whether `check` names a lint in `lints` (the toolchain's, from `parse_lints`, `-` already read as `_`), with every
/// `-` in `check` read as `_`. `warnings` is always known, listed or not: the check `warnings` takes every warning
/// (`parse_clippy`), whatever lints the toolchain has.
pub(crate) fn known_lint(check: &str, lints: &[String]) -> bool {
    check == "warnings" || lints.contains(&check.replace('-', "_"))
}
