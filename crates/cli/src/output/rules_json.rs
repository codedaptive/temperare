//! `rules_json`: the `tm rules` listing as one JSON object.

use serde_json::{json, Value};
use std::path::Path;

/// The rule file, its rules and its exceptions in force, in file order, with their count. Engine, kind and severity
/// take their serde names, which are the rule-file names. Each rule carries `available`, whether `availability` says
/// its engine can be run here, and `why` (the reason when not available, null when available).
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
                "available": why.is_none(),
                "why": why,
            })
        })
        .collect();
    let in_force = rf.exceptions_in_force();
    let exceptions: Vec<Value> = in_force
        .iter()
        .map(|e| {
            json!({
                "rule": e.rule,
                "path": e.path.as_str(),
                "what": e.what,
                "reason": e.reason,
                "approved": e.approved,
            })
        })
        .collect();
    json!({
        "rule_file": file.display().to_string(),
        "rules": rules,
        "exceptions": exceptions,
        "exceptions_in_force": in_force.len(),
    })
}
