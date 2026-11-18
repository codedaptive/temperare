//! `rules_json`: the `tm rules` listing as one JSON object.

use serde_json::{json, Map, Value};
use std::path::Path;

/// The rule file, its rules and its exceptions in force, in file order, with their count. Engine, kind and severity
/// take their serde names, which are the rule-file names. Each rule carries `available`, whether `availability` says
/// its engine can be run here, and `why` (the reason when not available, null when available); `outside_kits` is
/// always present, [] when the rule has none. An exception's `what` is present only when the exception names one.
pub fn rules_json(
    file: &Path,
    rf: &rules::RuleFile,
    availability: &dyn Fn(rules::Engine) -> engines::Availability,
) -> Value {
    let rules: Vec<Value> = rf
        .rules
        .iter()
        .map(|r| {
            let why = match availability(r.engine) {
                engines::Availability::Available => None,
                engines::Availability::NotAvailable { why } => Some(why),
            };
            json!({
                "id": r.id,
                "engine": r.engine,
                "kind": r.kind,
                "severity": r.severity,
                "check": r.check,
                "paths": r.paths,
                "exclude": r.exclude,
                "kits": r.kits,
                "outside_kits": r.outside_kits,
                "available": why.is_none(),
                "why": why,
            })
        })
        .collect();
    let in_force = rf.exceptions_in_force();
    let exceptions: Vec<Value> = in_force.iter().map(exception_json).collect();
    json!({
        "rule_file": file.display().to_string(),
        "rules": rules,
        "exceptions": exceptions,
        "exceptions_in_force": in_force.len(),
    })
}

/// One exception as an object with `rule`, `path`, `reason` and `approved`, and `what` only when it names one.
fn exception_json(e: &rules::Exception) -> Value {
    let mut object = Map::new();
    object.insert("rule".to_string(), json!(e.rule));
    object.insert("path".to_string(), json!(e.path.as_str()));
    if let Some(what) = &e.what {
        object.insert("what".to_string(), json!(what));
    }
    object.insert("reason".to_string(), json!(e.reason));
    object.insert("approved".to_string(), json!(e.approved));
    Value::Object(object)
}
