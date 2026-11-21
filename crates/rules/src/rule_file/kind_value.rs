//! `kind_value`: the `kind` of one `[[rule]]` table.

use super::problem_at::problem_at;
use super::string_value::string_value;
use crate::{Kind, Problem};
use toml::de::DeTable;

/// The rule's `kind`: Quality when absent; None (with a Problem) when it is not "quality" or "security".
pub(crate) fn kind_value(text: &str, table: &DeTable, out: &mut Vec<Problem>) -> Option<Kind> {
    if table.get("kind").is_none() {
        return Some(Kind::Quality);
    }
    let (name, at) = string_value(text, table, "kind", out)?;
    let kind = Kind::from_name(&name);
    if kind.is_none() {
        out.push(problem_at(text, at, format!("unknown kind \"{name}\""), "Use \"quality\" or \"security\"."));
    }
    kind
}
