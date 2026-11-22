//! `engine_value`: the `engine` of one `[[rule]]` table.

use super::problem_at::problem_at;
use super::required_string::required_string;
use crate::{Engine, Problem};
use toml::de::DeTable;

/// The table the key belongs to, named in the missing-key Problem.
const PLACE: &str = "[[rule]]";

/// The rule's `engine`, which must be one of the engine names; `at` is the byte offset of the rule's header,
/// where a missing `engine` is reported.
pub(crate) fn engine_value(text: &str, table: &DeTable, at: usize, out: &mut Vec<Problem>) -> Option<Engine> {
    let (name, name_at) = required_string(text, table, "engine", PLACE, at, out)?;
    let engine = Engine::from_name(&name);
    if engine.is_none() {
        let hint = format!("Use one of: {}.", Engine::ALL.map(Engine::name).join(", "));
        out.push(problem_at(text, name_at, format!("unknown engine \"{name}\""), &hint));
    }
    engine
}
