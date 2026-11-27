//! `lint_group`: whether a clippy check names a lint group rather than one lint.

/// True when `check`, with every `-` read as `_`, is one of `groups` (the toolchain's own list, `parse_lint_groups`)
/// and is not `warnings`, which is read as every warning rather than as a group. A group cannot be matched by code:
/// clippy reports each message under its member lint's code, cctao does not know which lints a group holds, and the
/// lints on by default warn in the same output, so a group's hits cannot be told from other warnings.
pub(crate) fn lint_group(check: &str, groups: &[String]) -> bool {
    let check = check.replace('-', "_");
    check != "warnings" && groups.contains(&check)
}
