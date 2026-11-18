//! `parse_lint_groups`: the lint group names, read from `clippy-driver -W help` output.

use super::help_names;
use crate::printed_start::printed_start;
use crate::types::ParseError;
use procio::Output;

/// The group names, in order, from every section whose title starts `Lint groups` (rustc's and clippy's), each with
/// `-` read as `_` (`help_names`). Other sections (the lint checks) are never read, so a lint is never taken for a
/// group. A non-zero exit, or help listing no group, is a `ParseError`: the group check must not pass for want of a
/// list.
pub(crate) fn parse_lint_groups(out: &Output) -> Result<Vec<String>, ParseError> {
    let names = if out.code == Some(0) { help_names(&String::from_utf8_lossy(&out.stdout), "Lint groups") } else { Vec::new() };
    if names.is_empty() {
        return Err(ParseError { why: format!("clippy-driver -W help listed no lint groups: {}", printed_start(out)) });
    }
    Ok(names)
}
