//! `rules_json`: the `tm rules` listing as one JSON object.

use super::split_exceptions::split_exceptions;
use serde_json::{json, Map, Value};
use std::path::Path;

/// The rule file, its rules, its exceptions in force on `day` (`exceptions`, counted in `exceptions_in_force`) and
/// those expired by `day` (`expired`, each with its `expires` as "YYYY-MM-DD", counted in `exceptions_expired`), each
/// list in file order, and `not_applicable` in the given order (each `{"rule", "engine", "why"}`, counted in
/// `rules_not_applicable`); `rf.exceptions_expired(day)` alone decides which are expired. Engine, kind and severity
/// take their serde names, which are the rule-file names. Each rule carries `available`, whether `availability` says
/// its engine can be run for the rule file's folder, and `why` (the reason when not available, null when available);
/// `outside_kits` is always present, [] when the rule has none. An exception's `what` is present only when the
/// exception names one.
pub fn rules_json(
    file: &Path,
    rf: &rules::RuleFile,
    availability: &dyn Fn(rules::Engine) -> engines::Availability,
    day: rules::Date,
    not_applicable: &[report::NotApplicable],
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
    let (in_force, expired_list) = split_exceptions(rf, day);
    let exceptions: Vec<Value> = in_force.into_iter().map(|e| Value::Object(exception_fields(e))).collect();
    let expired: Vec<Value> = expired_list.into_iter().map(expired_json).collect();
    json!({
        "rule_file": file.display().to_string(),
        "rules": rules,
        "exceptions_in_force": exceptions.len(),
        "exceptions": exceptions,
        "exceptions_expired": expired.len(),
        "expired": expired,
        "rules_not_applicable": not_applicable.len(),
        "not_applicable": not_applicable,
    })
}

/// An expired exception: `exception_fields` with its `expires` date as "YYYY-MM-DD" added.
fn expired_json((e, expires): (&rules::Exception, rules::Date)) -> Value {
    let mut object = exception_fields(e);
    object.insert("expires".to_string(), json!(expires.to_string()));
    Value::Object(object)
}

/// One exception's fields: `rule`, `path`, `reason` and `approved`, and `what` only when it names one.
fn exception_fields(e: &rules::Exception) -> Map<String, Value> {
    let mut object = Map::new();
    object.insert("rule".to_string(), json!(e.rule));
    object.insert("path".to_string(), json!(e.path.as_str()));
    if let Some(what) = &e.what {
        object.insert("what".to_string(), json!(what));
    }
    object.insert("reason".to_string(), json!(e.reason));
    object.insert("approved".to_string(), json!(e.approved));
    object
}
