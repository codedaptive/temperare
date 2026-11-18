//! `rules_text`: the `tm rules` listing as text.

use super::serde_name::serde_name;
use std::fmt::Write;
use std::path::Path;

/// The rule file's path, each rule with its engine, kind, severity, scope and whether `availability` says its engine
/// can be run here (`available`, or `not available: <why>`), then the exceptions in force, every line ending in a
/// newline. Both header lines print with their count, (0) included.
pub fn rules_text(
    file: &Path,
    rf: &rules::RuleFile,
    availability: &dyn Fn(rules::Engine) -> engines::Availability,
) -> String {
    let mut text = format!("rule file: {}\nrules ({}):\n", file.display(), rf.rules.len());
    for r in &rf.rules {
        let engine = serde_name(serde_json::to_value(r.engine));
        let kind = serde_name(serde_json::to_value(r.kind));
        let severity = serde_name(serde_json::to_value(r.severity));
        // Writing to a String cannot fail.
        let available = match availability(r.engine) {
            engines::Availability::Available => "available".to_string(),
            engines::Availability::NotAvailable { why } => format!("not available: {why}"),
        };
        let _ = writeln!(text, "  {}  {engine}  {kind}  {severity}  paths: {}  {available}", r.id, scope(r));
    }
    let in_force = rf.exceptions_in_force();
    let _ = writeln!(text, "exceptions in force ({}):", in_force.len());
    for e in in_force {
        let what = e.what.as_ref().map(|w| format!("what: {w}  ")).unwrap_or_default();
        let _ =
            writeln!(text, "  {}  {}  {what}approved: {}  reason: {}", e.rule, e.path.as_str(), e.approved, e.reason);
    }
    text
}

/// The paths joined ", " (`all files` when none), then `  exclude: ...`, `  kits: ...` and `  outside kits: ...`, each
/// only when its list is not empty.
fn scope(r: &rules::Rule) -> String {
    let mut scope = if r.paths.is_empty() { "all files".to_string() } else { r.paths.join(", ") };
    if !r.exclude.is_empty() {
        scope.push_str(&format!("  exclude: {}", r.exclude.join(", ")));
    }
    if !r.kits.is_empty() {
        scope.push_str(&format!("  kits: {}", r.kits.join(", ")));
    }
    if !r.outside_kits.is_empty() {
        scope.push_str(&format!("  outside kits: {}", r.outside_kits.join(", ")));
    }
    scope
}
