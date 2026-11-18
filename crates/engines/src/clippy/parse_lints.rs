//! `parse_lints`: the lint names, read from `clippy-driver -W help` output.

use super::help_names;
use crate::printed_start::printed_start;
use crate::types::ParseError;
use procio::Output;

/// The lint names, in order, from every section whose title starts `Lint checks` (rustc's and clippy's), each with
/// `-` read as `_` (`help_names`). The group sections are never read, so a name listed only as a group is never
/// taken for a lint (rustc lists `warnings` under both, so it is taken). A non-zero exit, or help listing no lint,
/// is a `ParseError`: the lint check must not pass for want of a list.
pub(crate) fn parse_lints(out: &Output) -> Result<Vec<String>, ParseError> {
    let names = if out.code == Some(0) { help_names(&String::from_utf8_lossy(&out.stdout), "Lint checks") } else { Vec::new() };
    if names.is_empty() {
        return Err(ParseError { why: format!("clippy-driver -W help listed no lints: {}", printed_start(out)) });
    }
    Ok(names)
}
