//! `check_json`: tm check's `--json` object.

use report::Report;
use serde_json::{Map, Value};

/// The report's fields (findings, silenced, exceptions_in_force, not_available, failed, base when there is one,
/// covers_nothing, not_applicable, date, silenced_by_rule, engines, counts) with `exit` and `next` added at the top
/// level.
pub fn check_json(report: &Report, exit: u8, next: &str) -> Value {
    // A Report always serializes to an object; the empty map only keeps this total without an unwrap.
    let mut object = match serde_json::to_value(report) {
        Ok(Value::Object(map)) => map,
        _ => Map::new(),
    };
    object.insert("exit".to_string(), Value::from(exit));
    object.insert("next".to_string(), Value::from(next));
    Value::Object(object)
}
